use std::time::Duration;
use log::{debug, info};
use rdkafka::{ClientConfig, Message};
use rdkafka::consumer::{BaseConsumer, Consumer};
use serde::__private::from_utf8_lossy;
use crate::config::CONFIG;
use crate::models::Person;

pub fn consumer(client_config: ClientConfig) {
    // Create a new Kafka consumer using the provided client configuration
    let consumer: BaseConsumer = client_config.create().expect("Consumer creation failed");

    // Subscribe to the "demo_topic" Kafka topic
    consumer
        .subscribe(&[&CONFIG.topic])
        .expect("Subscription to topic failed");

    // Start an infinite loop for polling messages from Kafka
    loop {
        match consumer.poll(Duration::from_millis(10)) {
            Some(Ok(message)) => {
                // If a message is received successfully, extract and process its payload
                if let Some(payload) = message.detach().payload() {
                    // Deserialize the payload assuming it's UTF-8 encoded
                    let person: Person = serde_json::from_slice(payload)
                        .expect("Unable to deserialize data");

                    // Log the deserialized person object
                    info!("{:#?}", person);
                }
            }
            Some(Err(err)) => {
                // If an error occurs during message consumption, print the error
                eprintln!("Error: {:?}", err);
            }
            _ => {} // Do nothing if no message is received
        }
    }
}
