#[macro_use]
use crate::macros::attempt;

use std::error::Error;
use std::fs;
use rbatis::rbatis::Rbatis;

pub async fn create() {
    let connectionstring = load_connection_string();
    crate::database::RB.link("postgres://postgres:postgres@localhost:5432/epat_webdb").await.unwrap();
}

fn load_connection_string() -> String {
    let content :String;
    match load_config_file() {
        Ok(config) => content = config,
        Err(err) => {
            let emptyConfig =  Config {
                address: String(),
                server: String(),
                username: String(),
                password: String(),
            };

            fs::write("./config", serde_json)
        }
    }


    return "".to_string();
}

fn load_config_file() -> Result<String, dyn Error>{
    let content = fs::read_to_string("./config.json").expect("Error loading file");
    Ok(content)
}

struct Config {
    pub address: String,
    pub server: String,
    pub username: String,
    pub password: String,
}
