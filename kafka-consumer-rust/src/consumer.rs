use std::time::Duration;
use log::{debug, info};
use rdkafka::{ClientConfig, Message};
use rdkafka::consumer::{BaseConsumer, Consumer};
use serde::__private::from_utf8_lossy;
use crate::config::CONFIG;
use crate::models::Person;

pub fn consumer(client_config: ClientConfig) {
    let consumer: BaseConsumer = client_config.create().expect("Consumer creation failed");
    consumer
        .subscribe(&["demo_topic"])
        .expect("Subscription to topic failed");
    loop {
        match consumer.poll(Duration::from_millis(10)) {
            Some(Ok(message)) => {
                if let Some(payload) = message.detach().payload() {
                    // Deserialize the payload assuming it's UTF-8 encoded
                    let person :Person= serde_json::from_slice(payload).expect("unable to deserialize data");
                    info!("{:#?}",person);
                }
            }
            Some(Err(err)) => eprintln!("Error: {:?}", err),
            _=> {}
        }
    }
}