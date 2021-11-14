#![deny(warnings)]
#[macro_use]
extern crate rbatis;
extern crate lazy_static;

mod request_handling;
mod database;
mod responses;
mod logger;

use async_std::net::TcpListener;
use futures::stream::StreamExt;

use std::panic;
use async_std::task::spawn;
use log::LevelFilter;
static LOGGER: logger::Logger = logger::Logger;

#[async_std::main]
async fn main() {
    let set_logger_result = log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Debug));
    if set_logger_result.is_err(){
        panic!("{}", set_logger_result.unwrap_err().to_string());
    }

    log::info!("Connecting to sql server");
    database::connection::create().await;

    log::info!("Starting a new redirect server");
    let listener = TcpListener::bind("0.0.0.0:7878").await.unwrap();


    listener.incoming().for_each_concurrent(None, |stream| async move {
        let stream = stream.unwrap();
        spawn(request_handling::handle_connection(stream));
    }).await;
}
