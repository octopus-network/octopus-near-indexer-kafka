use std::time::Duration;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use crate::kafka::config::get_config;

pub async fn produce(topic_name: &str, json: &String) {
    let producer: &FutureProducer = &get_config().expect("")
        .create()
        .expect("");

    let delivery_status = producer
        .send(
            FutureRecord::to(topic_name)
                .payload(&format!("{}", json))
                .key(&format!("{}", "2")),
            Timeout::Never,
        )
        .await;

    tracing::info!("Future completed. Result: {:?}", delivery_status);
}