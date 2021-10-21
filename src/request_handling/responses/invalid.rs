use std::str;
use log::{info, trace, warn};


pub fn response() -> String {
    info!("Creating 400 bad reqiest");

    let content = r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Bad request</title>
  </head>
  <body>
    <h1>Bad request</h1>
  </body>
</html>"#;

    let response = format!(
        "HTTP/1.1 400 Bad Request\r\nContent-Length: {}\r\n\r\n{}",
        content.len(),
        content
    );

    return response;
}