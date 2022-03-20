use anyhow::{Error, Result};
use rdkafka::config::ClientConfig;
use std::env;

pub mod produce;

pub fn kafka_config() -> Result<ClientConfig> {
    let mut kafka_config = ClientConfig::new();
    for (key, value) in env::vars() {
        if !key.starts_with("kafka.") || key.len() < 1 {
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
