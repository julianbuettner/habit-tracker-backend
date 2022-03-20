use serde::Deserialize;
use serde_yaml::from_str;
use std::{env, fs};

#[derive(Deserialize, Debug, Clone)]
pub struct Configuration {
    pub postgres: String,
    #[serde(rename = "auth-whitelist")]
    pub auth_whitelist: Vec<String>,
}

pub fn read_config() -> Configuration {
    let home = env::var("HOME").expect("HOME variable undefined");
    let content = fs::read_to_string(&format!("{}/.habit-tracker.yaml", home))
        .expect("Failed to read ~/.habit-tracker.yaml");
    from_str(&content).expect("Failed to parse ~/.habit-tracker.yaml")
}
