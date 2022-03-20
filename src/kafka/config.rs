use rdkafka::config::ClientConfig;
use std::boxed::Box;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn get_config() -> Result<ClientConfig, Box<dyn std::error::Error>> {

    let mut kafka_config = ClientConfig::new();

    let file = File::open("./.env")?;
    for line in BufReader::new(&file).lines() {
        let cur_line: String = line?.trim().to_string();
        if cur_line.starts_with('#') || cur_line.len() < 1 {
            continue;
        }
        let key_value: Vec<&str> = cur_line.split("=").collect();
        kafka_config.set(
            key_value.get(0).ok_or("malformed key")?.to_owned(),
            key_value.get(1).ok_or("malformed value")?.to_owned(),
        );
    }

    Ok(kafka_config)
}