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
pub struct Opts {
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
    /// Initialize boots nodes in <node_key>@<ip_addr> format seperated by commas
    /// to bootstrap the network and store them in config.json
    #[clap(long)]
    pub boot_nodes: Option<String>,
}