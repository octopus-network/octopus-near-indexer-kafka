use rdkafka::config::ClientConfig;
use std::env;

pub mod produce;

pub fn get_config() -> Result<ClientConfig, Box<dyn std::error::Error>> {
    let mut kafka_config = ClientConfig::new();
    for (key, value) in env::vars() {
        if !key.starts_with("kafka.") || key.len() < 1 {
            continue;
        }
        let config_key: Vec<&str> = key.split("kafka.").collect();
        kafka_config.set(config_key.get(1).ok_or("malformed key")?.to_owned(), value);
    }
    Ok(kafka_config)
}
