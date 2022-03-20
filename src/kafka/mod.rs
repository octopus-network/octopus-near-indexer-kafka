use anyhow::{Error, Result};
use rdkafka::config::ClientConfig;
use rdkafka::producer::FutureProducer;
use std::env;

pub mod produce;

pub fn kafka_config() -> Result<ClientConfig> {
    let mut kafka_config = ClientConfig::new();
    for (key, value) in env::vars() {
        if !key.starts_with("kafka.") || key.is_empty() {
            continue;
        }
        let config_key: Vec<&str> = key.split("kafka.").collect();
        kafka_config.set(
            config_key
                .get(1)
                .ok_or("fail load value")
                .map_err(Error::msg)?
                .to_owned(),
            value,
        );
    }
    Ok(kafka_config)
}

pub fn try_producer() -> Result<FutureProducer> {
    let config = kafka_config()?;
    let producer = config.create()?;
    Ok(producer)
}
