use rdkafka::config::ClientConfig;
use rdkafka::consumer::Consumer;
use rdkafka::Message;
use rdkafka::producer::BaseProducer;
use config::CONFIG;

use crate::model::Person;
use crate::producer::producer;

mod config;
mod model;
mod producer;

fn main() {
   env_logger::init();
   let client_config:BaseProducer=ClientConfig::new()
       .set("bootstrap.servers",&CONFIG.kafka_broker)
       .create()
       .expect("Producer creation error");

   let person_data = Person{
      name: "Sheshnath".to_string(),
      age: 23,
      gender: "Male".to_string(),
   };

   producer(client_config,person_data);
}
