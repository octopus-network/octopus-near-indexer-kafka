use crate::kafka::try_producer;
use crate::models::cli::INDEXER;
use rdkafka::producer::FutureRecord;
use rdkafka::util::Timeout;

pub async fn send(topic_name: &str, streamer_message: &near_indexer::StreamerMessage) {
    let json =
        serde_json::to_value(&streamer_message).expect("Failed to serializer message to JSON");

    let delivery_status = try_producer()
        .expect("Fail get producer")
        .send(
            FutureRecord::to(topic_name)
                .payload(&json.to_string())
                .key(&json.to_string()),
            Timeout::Never,
        )
        .await;

    // TODO: Send fail retry

    tracing::info!(
        target: INDEXER,
        "Future completed. Result: {:?}. Block: {:?}",
        delivery_status,
        format!("{:0>12}", streamer_message.block.header.height)
    );
}
