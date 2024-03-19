use config::CONFIG;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::Consumer;
use rdkafka::producer::BaseProducer;
use rdkafka::Message;

use crate::model::Person;
use crate::producer::producer;

mod config;
mod model;
mod producer;

fn main() {
    // Initialize logger for logging
    env_logger::init();
    // Create a new BaseProducer with the specified bootstrap servers from CONFIG
    let client_config: BaseProducer = ClientConfig::new()
        .set("bootstrap.servers", &CONFIG.kafka_broker)
        .create()
        .expect("Producer creation error");
    // Create a Person instance with dummy data
    let person_data = Person {
        name: "Sheshnath".to_string(),
        age: 23,
        gender: "Male".to_string(),
    };

    // Call the producer function with the created BaseProducer and Person data
    producer(client_config, person_data);
}
