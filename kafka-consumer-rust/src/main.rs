use rdkafka::config::ClientConfig;
use rdkafka::consumer::Consumer;
use rdkafka::Message;
use crate::config::CONFIG;
use crate::consumer::consumer;

mod config;
mod models;
mod consumer;

fn main() {
    env_logger::init(); // Initialize the logger

    // Create a new client configuration
    let mut client_config = ClientConfig::new();

    // Set the group ID and bootstrap servers from the CONFIG struct
    client_config
        .set("group.id", &CONFIG.group_id)
        .set("bootstrap.servers", &CONFIG.kafka_broker);

    // Call the consumer function with the configured client configuration
    consumer(client_config);
}
