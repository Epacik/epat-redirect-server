#[macro_use]
use crate::macros::attempt;

use std::error::Error;
use std::fs;
use rbatis::rbatis::Rbatis;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value};

pub async fn create() {
    let connectionstring = load_connection_string();
    crate::database::RB.link(&*connectionstring).await.unwrap();
}

fn load_connection_string() -> String {
    let content :String;
    match load_config_file() {
        Ok(config) => content = config,
        Err(err) => {
            let emptyConfig =  Config {
                address: "".to_string(),
                username: "".to_string(),
                password: "".to_string(),
                database: "".to_string(),
            };

            match serde_json::to_vec_pretty(&emptyConfig){
                Ok(value) => {
                    fs::write("./config.json", value);
                    panic!();
                },
                Err(err) => {
                    panic!();
                },
            }
        }
    }

    let config: Config = from_str(&*content).unwrap();


    return format!("postgres://{}:{}@{}:5432/{}", config.username, config.password, config.address, config.database);
}

fn load_config_file() -> Result<String, Box<dyn Error>>{
    let content = fs::read_to_string("./config.json")?;
    Ok(content)
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub address: String,
    pub username: String,
    pub password: String,
    pub database: String,
}
