use tracing_subscriber::EnvFilter;

pub fn init_tracing() {
    let env_filter = EnvFilter::new(
        "indexer=info,octopus_near_indexer=trace,chain=info,network=error,stats=info,near_network=error",
    );

    tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(env_filter)
        .init();
}
