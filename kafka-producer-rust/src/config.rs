use hocon::HoconLoader;
use lazy_static::lazy_static;
use serde::Deserialize;

// Derive the Deserialize and Debug traits for the Config struct
#[derive(Deserialize, Debug)]
pub struct Config {
    pub kafka_broker: String,
    pub topic: String,
    pub group_id: String,
}

// Define a static CONFIG variable initialized with the result of the get_config function
lazy_static! {
    pub static ref CONFIG: Config = get_config();
}

// Load and resolve the HOCON configuration file
fn get_config() -> Config {
    let config: Config = HoconLoader::new()
        .load_file("src/resource/application.config")
        .expect("unable to load config file")
        .resolve()
        .expect("config deserialize error");
    config
}
