use clap::Parser;
use dotenv::dotenv;
use futures::StreamExt;
use octopus_near_indexer_kafka::kafka::produce::send;
use octopus_near_indexer_kafka::log::init_tracing;
use octopus_near_indexer_kafka::models::cli::{IndexerOpts, RunSubCommand, INDEXER};

fn main() {
    dotenv().ok();
    // We use it to automatically search the for root certificates to perform HTTPS calls
    // (sending telemetry and downloading genesis)
    openssl_probe::init_ssl_cert_env_vars();

    init_tracing();

    let opts: IndexerOpts = IndexerOpts::parse();

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

                listen_blocks(stream, args.concurrency).await;

                actix::System::current().stop();
            });
            system.run().unwrap();
        }
    }
}

async fn listen_blocks(
    stream: tokio::sync::mpsc::Receiver<near_indexer::StreamerMessage>,
    concurrency: std::num::NonZeroU16,
) {
    let mut handle_messages = tokio_stream::wrappers::ReceiverStream::new(stream)
        .map(|streamer_message| {
            tracing::info!(
                target: INDEXER,
                "Block height {}",
                &streamer_message.block.header.height
            );
            handle_message(streamer_message)
        })
        .buffer_unordered(usize::from(concurrency.get()));

    while let Some(_handle_message) = handle_messages.next().await {}
}

async fn handle_message(
    streamer_message: near_indexer::StreamerMessage,
) -> anyhow::Result<()> {
    send("blockchain-near-analysis", &streamer_message).await;
    Ok(())
}
