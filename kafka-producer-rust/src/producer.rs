use std::time::Duration;
use log::debug;
use rdkafka::producer::{BaseProducer, BaseRecord};
use crate::config::CONFIG;
use crate::model::Person;

pub fn producer(producer:BaseProducer, person_data: Person){
    for _ in 1..10{
        producer.send(
            BaseRecord::to(&CONFIG.topic)
                .payload(serde_json::to_string(&person_data).unwrap().as_bytes())
                .key("and this is a key"),
        ).expect("Failed to enqueue");
        producer.poll(Duration::from_millis(100));
        debug!("producing");
    }
}
