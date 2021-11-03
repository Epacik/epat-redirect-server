#[macro_use]
extern crate rbatis;
extern crate lazy_static;

mod request_handling;
mod database;
mod macros;
mod responses;

use std::io::prelude::*;
use async_std::net::TcpListener;
use async_std::net::TcpStream;
use futures::stream::StreamExt;

use std::{panic, thread, time};
use async_std::task::spawn;
use log::{info, trace, warn};

#[async_std::main]
async fn main() {
    database::connection::create().await;

    info!("Starting a new redirect server");
    let listener = TcpListener::bind("0.0.0.0:7878").await.unwrap();


    listener.incoming().for_each_concurrent(None, |stream| async move {
        let stream = stream.unwrap();
        spawn(request_handling::handle_connection(stream));
    }).await;
}
