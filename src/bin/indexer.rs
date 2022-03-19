use std::sync::Arc;

use clap::Parser;
use futures::StreamExt;
use tokio::sync::Mutex;
use octopus_near_indexer_kafka::models::cli::indexer::{Opts, RunSubCommand, Stats, INDEXER};
use octopus_near_indexer_kafka::log::{init_tracing, indexer_logger};

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
                "NEAR Indexer Kafka v{} starting...",
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
                ).await;

                actix::System::current().stop();
            });
            system.run().unwrap();
        }
    }
}


async fn listen_blocks(
    stream: tokio::sync::mpsc::Receiver<near_indexer_primitives::StreamerMessage>,
    concurrency: std::num::NonZeroU16,
    stats: Arc<Mutex<Stats>>,
) {
    let mut handle_messages = tokio_stream::wrappers::ReceiverStream::new(stream)
        .map(|streamer_message| {
            tracing::info!(
                "Block height {}", &streamer_message.block.header.height
            );

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
    let block_json = serde_json::to_value(streamer_message.block)
        .expect("Failed to serializer BlockView to JSON");
    // TODO: Push Block to Kafka
    println!("{}", block_json);

    // Shards
    for shard in streamer_message.shards.iter() {
        let _key = format!("{}/shard_{}.json", base_key, shard.shard_id);
        let shard_json =
            serde_json::to_value(shard).expect("Failed to serialize IndexerShard to JSON");
        // TODO: Push Block to Kafka
        println!("{}", shard_json);
    }
    let mut stats_lock = stats.lock().await;
    stats_lock.block_heights_processing.remove(&block_height);
    stats_lock.blocks_processed_count += 1;
    stats_lock.last_processed_block_height = block_height;
    drop(stats_lock);
    Ok(())
}