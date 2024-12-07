use serde::Deserialize;

#[derive(Deserialize, Debug, Default, PartialEq, Eq)]
pub struct Config {
    pub address: String,
    pub pg_url: Option<String>,
}

impl Config {
    pub fn new() -> Config {
        config::Config::builder()
            .add_source(config::Environment::default()
                .try_parsing(true)
                .separator("_"))
            .build()
            .expect("Failed to parse configuration from env")
            .try_deserialize()
            .expect("Failed to deserialize configuration from env")
    }
}
