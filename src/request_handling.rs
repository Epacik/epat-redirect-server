mod responses;

use std::borrow::Borrow;
use std::str;
use async_std::net::TcpListener;
use async_std::net::TcpStream;
use futures::{AsyncReadExt, AsyncWriteExt};
use log::{info, trace, warn};
use rbatis::crud::CRUD;
use crate::database;


pub(crate) async fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).await.unwrap();

    let request_str: &str = str::from_utf8(&buffer).unwrap();

    let response : String = get_response(request_str.to_string()).await;

    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}

async fn get_response(request: String) -> String {
    if ! is_request_valid(request.clone()) {
        return responses::invalid_response();
    }

    let slash_position_option = request.find("/");
    let http_position_option = request.find("HTTP");
    let slash_position : usize;
    let http_position : usize;

    match slash_position_option {
        Some(pos) => slash_position = pos,
        None            => slash_position = 0,
    }
    match http_position_option {
        Some(pos) => http_position = pos,
        None            => http_position = 0,
    }

    let path = request
            .split_at(http_position)     .0
            .split_at(slash_position + 1).1
            .trim();

    let result_option: Option<database::links::Links> = database::RB.fetch_by_column("lnk_path", &path).await.unwrap();
    match result_option {
        Some(res) => return responses::redirect_response(res).await,
        None => return responses::not_found_response(),
    }
}

fn is_request_valid(request: String) -> bool {
    if(!request.starts_with("GET") || request.starts_with("GET / "))
    {
        return false;
    }

    return true;
}


