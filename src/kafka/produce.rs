use crate::kafka::kafka_config;
use crate::models::cli::INDEXER;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;

pub async fn produce(topic_name: &str, json: &String) {
    let producer: &FutureProducer = &kafka_config()
        .expect("Kafka config check fail")
        .create()
        .expect("Kafka config build fail");

    let delivery_status = producer
        .send(
            FutureRecord::to(topic_name)
                .payload(&format!("{}", json))
                .key(&format!("{}", "1")),
            Timeout::Never,
        )
        .await;

    tracing::info!(
        target: INDEXER,
        "Future completed. Result: {:?}",
        delivery_status
    );
}
