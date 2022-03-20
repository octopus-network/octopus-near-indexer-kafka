use crate::kafka::try_producer;
use crate::models::cli::INDEXER;
use rdkafka::producer::FutureRecord;
use rdkafka::util::Timeout;

pub async fn produce(topic_name: &str, json: &str) {
    let delivery_status = try_producer()
        .expect("Fail get producer")
        .send(
            FutureRecord::to(topic_name)
                .payload(&json.to_string())
                .key(&json.to_string()),
            Timeout::Never,
        )
        .await;

    tracing::info!(
        target: INDEXER,
        "Future completed. Result: {:?}",
        delivery_status
    );
}
