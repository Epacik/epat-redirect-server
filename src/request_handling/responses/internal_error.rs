use std::str;
use log::{info, trace, warn};


pub fn response() -> String {
    info!("Creating 500 Internal server error");

    let content = r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Internal server error/title>
  </head>
  <body>
    <h1>Internal server error</h1>
  </body>
</html>"#;

    let response = format!(
        "HTTP/1.1 500 Internal Server Error\r\nContent-Length: {}\r\n\r\n{}",
        content.len(),
        content
    );

    return response;
}