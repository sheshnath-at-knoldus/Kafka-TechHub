use rdkafka::config::ClientConfig;
use rdkafka::consumer::Consumer;
use rdkafka::Message;
use crate::config::CONFIG;
use crate::consumer::consumer;
mod config;
mod models;
mod consumer;

fn main() {
    env_logger::init();
    let mut client_config = ClientConfig::new();
    client_config
        .set("group.id", &CONFIG.group_id)
        .set("bootstrap.servers", &CONFIG.kafka_broker);
    consumer(client_config);
}

