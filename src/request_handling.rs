use std::str;
use async_std::net::TcpStream;
use futures::{AsyncReadExt, AsyncWriteExt};
use log::{info, trace, warn};
use rbatis::crud::CRUD;
use crate::database;
use crate::responses::{invalid_response, not_found_response, redirect_response};


pub(crate) async fn handle_connection(mut stream: TcpStream) {
    info!("przetwarzanie zapytania od: {}", stream.peer_addr().unwrap());
    let response : String;

    let mut buffer = [0; 1024];

    stream.read(&mut buffer).await.unwrap();

    let request_str: &str = str::from_utf8(&buffer).unwrap();

    response = get_response(request_str.to_string()).await;

    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}

async fn get_response(request: String) -> String {
    if ! is_request_valid(request.clone()) {
        info!("niepoprawne zapytanie");
        return invalid_response();
    }

    // wyszukujemy pozycje pierwszego ukośnika oraz informacji o wersji http
    // zakładając, że zapytanie http wygląda mniej więcej tak:
    // "GET /test HTTP/1.1
    // Host: [tu wstaw adres serwera]
    // user-agent: [tu wstaw user-agent]
    // accept: */*"
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

    //interesują nas te konkretne pozycje, poniważ pozwalają one na wyizolowanie ścieżki do zasobu
    // w przypadku zapytania pokazanego powyżej, będzie to "test"
    // w przypadku poniższego zapytania będzie to "jakas/wieloczesciowa/sciezka"
    // GET /jakas/wieloczesciowa/sciezka HTTP/1.1
    let path = request
            .split_at(http_position)     .0 //najpierw dzielimy ciąg w w miejscu w którym znajduje się "HTTP", i bierzemy część od początku tekstu, do HTTP
            .split_at(slash_position + 1).1 //Następnie dzielimy ciąg w miejscu pierwszego ukośnika, i bierzemy część od ukośnika, do końca tekstu
            .trim(); // przycinamy niepotrzebne białe znaki z początkui i końca tekstu

    let result_option: Option<database::links::Links> = database::RB.fetch_by_column("lnk_path", &path).await.unwrap();

    if result_option.is_none(){
        return not_found_response();
    }

    return redirect_response(res).await
}

fn is_request_valid(request: String) -> bool {
    if(!request.starts_with("GET") || request.starts_with("GET / "))
    {
        return false;
    }

    return true;
}


