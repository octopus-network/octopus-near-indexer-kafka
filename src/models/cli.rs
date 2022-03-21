use clap::Parser;

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
pub struct GenerateOpts {
    /// Sets a custom config dir. Defaults to ~/.near/
    #[clap(short, long)]
    pub home: Option<std::path::PathBuf>,
    #[clap(subcommand)]
    pub sub_cmd: GenerateSubCommand,
}

#[derive(Parser, Clone, Debug)]
pub enum GenerateSubCommand {
    /// Initialize necessary configs
    Init(InitConfigArgs),
}

#[derive(Parser, Clone, Debug)]
pub struct InitConfigArgs {
    /// chain/network id (localnet, testnet, devnet, betanet)
    #[clap(short, long)]
    pub chain_id: Option<String>,
    /// Account ID for the validator key
    #[clap(long)]
    pub account_id: Option<String>,
    /// Specify private key generated from seed (TESTING ONLY)
    #[clap(long)]
    pub test_seed: Option<String>,
    /// Number of shards to initialize the chain with
    #[clap(short, long, default_value = "1")]
    pub num_shards: u64,
    /// Makes block production fast (TESTING ONLY)
    #[clap(short, long)]
    pub fast: bool,
    /// Genesis file to use when initialize testnet (including downloading)
    #[clap(short, long)]
    pub genesis: Option<String>,
    #[clap(short, long)]
    /// Download the verified NEAR config file automatically.
    #[clap(long)]
    pub download_config: bool,
    #[clap(long)]
    pub download_config_url: Option<String>,
    /// Download the verified NEAR genesis file automatically.
    #[clap(long)]
    pub download_genesis: bool,
    /// Specify a custom download URL for the genesis-file.
    #[clap(long)]
    pub download_genesis_url: Option<String>,
    /// Customize max_gas_burnt_view runtime limit.  If not specified, value
    /// from genesis configuration will be taken.
    #[clap(long)]
    pub max_gas_burnt_view: Option<u64>,
    /// Initialize boots nodes in <node_key>@<ip_addr> analysis seperated by commas
    /// to bootstrap the network and store them in config.json
    #[clap(long)]
    pub boot_nodes: Option<String>,
}

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
pub struct IndexerOpts {
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
    pub fn to_indexer_config(self, home_dir: std::path::PathBuf) -> near_indexer::IndexerConfig {
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

pub const INDEXER: &str = "octopus_near_indexer";

#[derive(Debug, Clone)]
pub struct Stats {
    pub block_heights_processing: std::collections::BTreeSet<u64>,
    pub blocks_processed_count: u64,
    pub last_processed_block_height: u64,
}

impl Default for Stats {
    fn default() -> Self {
        Self::new()
    }
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
