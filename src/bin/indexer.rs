use std::sync::Arc;

use clap::Parser;
use futures::StreamExt;
use tokio::sync::Mutex;
use octopus_near_indexer_kafka::tracing::init_tracing;
use octopus_near_indexer_kafka::utils;

/// Octopus Near Indexer Kafka
/// Watches for stream of blocks from the chain and puts it in Kafka
#[derive(Parser, Clone, Debug)]
#[clap(
version,
author,
about,
setting(clap::AppSettings::DisableHelpSubcommand),
setting(clap::AppSettings::PropagateVersion),
setting(clap::AppSettings::NextLineHelp)
)]
pub struct Opts {
    /// Sets a custom config dir. Defaults to ~/.near/
    #[clap(short, long)]
    pub home: Option<std::path::PathBuf>,
    #[clap(subcommand)]
    pub sub_cmd: RunSubCommand,
}

#[derive(Parser, Clone, Debug)]
pub enum RunSubCommand {
    /// Run NEAR Indexer. Start observe the network
    Run(RunArgs),
}

#[derive(Parser, Debug, Clone)]
pub struct RunArgs {
    /// Force streaming while node is syncing
    #[clap(long)]
    pub stream_while_syncing: bool,
    /// Sets the concurrency for indexing. Note: concurrency (set to 2+) may lead to warnings due to tight constraints between transactions and receipts (those will get resolved eventually, but unless it is the second pass of indexing, concurrency won't help at the moment).
    #[clap(long, default_value = "1")]
    pub concurrency: std::num::NonZeroU16,
    /// Sets the starting point for indexing
    #[clap(subcommand)]
    pub sync_mode: SyncModeSubCommand,
}

impl RunArgs {
    pub(crate) fn to_indexer_config(
        self,
        home_dir: std::path::PathBuf,
    ) -> near_indexer::IndexerConfig {
        near_indexer::IndexerConfig {
            home_dir,
            sync_mode: self.sync_mode.into(),
            await_for_node_synced: if self.stream_while_syncing {
                near_indexer::AwaitForNodeSyncedEnum::StreamWhileSyncing
            } else {
                near_indexer::AwaitForNodeSyncedEnum::WaitForFullSync
            },
        }
    }
}

const INDEXER: &str = "near_lake";

#[derive(Debug, Clone)]
struct Stats {
    pub block_heights_processing: std::collections::BTreeSet<u64>,
    pub blocks_processed_count: u64,
    pub last_processed_block_height: u64,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            block_heights_processing: std::collections::BTreeSet::new(),
            blocks_processed_count: 0,
            last_processed_block_height: 0,
        }
    }
}

#[allow(clippy::enum_variant_names)] // we want commands to be more explicit
#[derive(Parser, Debug, Clone)]
pub enum SyncModeSubCommand {
    /// continue from the block Indexer was interrupted
    SyncFromInterruption,
    /// start from the newest block after node finishes syncing
    SyncFromLatest,
    /// start from specified block height
    SyncFromBlock(BlockArgs),
}

#[derive(Parser, Debug, Clone)]
pub struct BlockArgs {
    /// block height for block sync mode
    #[clap(long)]
    pub height: u64,
}

impl From<SyncModeSubCommand> for near_indexer::SyncModeEnum {
    fn from(sync_mode: SyncModeSubCommand) -> Self {
        match sync_mode {
            SyncModeSubCommand::SyncFromInterruption => Self::FromInterruption,
            SyncModeSubCommand::SyncFromLatest => Self::LatestSynced,
            SyncModeSubCommand::SyncFromBlock(args) => Self::BlockHeight(args.height),
        }
    }
}

fn main() {
    // We use it to automatically search the for root certificates to perform HTTPS calls
    // (sending telemetry and downloading genesis)
    openssl_probe::init_ssl_cert_env_vars();
    init_tracing();

    let opts: Opts = Opts::parse();

    let home_dir = opts.home.unwrap_or_else(near_indexer::get_default_home);

    match opts.sub_cmd {
        RunSubCommand::Run(args) => {
            tracing::info!(
                target: INDEXER,
                "NEAR Lake v{} starting...",
                env!("CARGO_PKG_VERSION")
            );

            let system = actix::System::new();
            system.block_on(async move {
                let indexer_config = args.clone().to_indexer_config(home_dir);
                let indexer = near_indexer::Indexer::new(indexer_config)
                    .expect("Failed to initialize the Indexer");

                // Regular indexer process starts here
                let stream = indexer.streamer();
                let view_client = indexer.client_actors().0;

                let stats: Arc<Mutex<Stats>> = Arc::new(Mutex::new(Stats::new()));

                actix::spawn(indexer_logger(Arc::clone(&stats), view_client));

                listen_blocks(
                    stream,
                    args.concurrency,
                    Arc::clone(&stats),
                )
                    .await;

                actix::System::current().stop();
            });
            system.run().unwrap();
        }
    }
}

async fn indexer_logger(
    stats: Arc<Mutex<Stats>>,
    view_client: actix::Addr<near_client::ViewClientActor>,
) {
    let interval_secs = 10;
    let mut prev_blocks_processed_count: u64 = 0;

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(interval_secs)).await;
        let stats_lock = stats.lock().await;
        let stats_copy = stats_lock.clone();
        drop(stats_lock);

        let block_processing_speed: f64 = ((stats_copy.blocks_processed_count
            - prev_blocks_processed_count) as f64)
            / (interval_secs as f64);

        let time_to_catch_the_tip_duration =
            if let Ok(block_height) = utils::fetch_latest_block(&view_client).await {
                Some(std::time::Duration::from_millis(
                    if  block_height == 0 && stats_copy.last_processed_block_height == 0 && block_processing_speed == 0.0 {
                        0
                    } else {
                        (block_height - stats_copy.last_processed_block_height)
                            / ((block_processing_speed * 1_000f64) as u64)
                    }
                ))
            } else {
                None
            };

        tracing::info!(
            target: INDEXER,
            "# {} | Blocks processing: {}| Blocks done: {}. Bps {:.2} b/s {}",
            stats_copy.last_processed_block_height,
            stats_copy.block_heights_processing.len(),
            stats_copy.blocks_processed_count,
            block_processing_speed,
            if let Some(duration) = time_to_catch_the_tip_duration {
                format!(
                    " | {} to catch up the tip",
                    humantime::format_duration(duration)
                )
            } else {
                "".to_string()
            }
        );
        prev_blocks_processed_count = stats_copy.blocks_processed_count;
    }
}


async fn listen_blocks(
    stream: tokio::sync::mpsc::Receiver<near_indexer_primitives::StreamerMessage>,
    concurrency: std::num::NonZeroU16,
    stats: Arc<Mutex<Stats>>,
) {
    let mut handle_messages = tokio_stream::wrappers::ReceiverStream::new(stream)
        .map(|streamer_message| {
            handle_message(
                streamer_message,
                Arc::clone(&stats),
            )
        })
        .buffer_unordered(usize::from(concurrency.get()));

    while let Some(_handle_message) = handle_messages.next().await {}
}

async fn handle_message(
    streamer_message: near_indexer_primitives::StreamerMessage,
    stats: Arc<Mutex<Stats>>,
) -> anyhow::Result<()> {
    let block_height = streamer_message.block.header.height;
    let mut stats_lock = stats.lock().await;
    stats_lock.block_heights_processing.insert(block_height);
    drop(stats_lock);

    let base_key = format!("{:0>12}", streamer_message.block.header.height);

    // Block
    let _block_json = serde_json::to_value(streamer_message.block)
        .expect("Failed to serializer BlockView to JSON");
    // TODO: Push BlockJson

    // Shards
    for shard in streamer_message.shards.iter() {
        let _key = format!("{}/shard_{}.json", base_key, shard.shard_id);
        let _shard_json =
            serde_json::to_value(shard).expect("Failed to serialize IndexerShard to JSON");
        // TODO: Push ShardJson
    }
    let mut stats_lock = stats.lock().await;
    stats_lock.block_heights_processing.remove(&block_height);
    stats_lock.blocks_processed_count += 1;
    stats_lock.last_processed_block_height = block_height;
    drop(stats_lock);
    Ok(())
}