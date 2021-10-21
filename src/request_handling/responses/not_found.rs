use std::str;
use log::{info, trace, warn};


pub fn response() -> String {
    info!("Creating 404 not found response");

    let content = r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Link was not found</title>
  </head>
  <body>
    <h1>Link was not found</h1>
  </body>
</html>"#;

    let response = format!(
        "HTTP/1.1 404 Not Found\r\nContent-Length: {}\r\n\r\n{}",
        content.len(),
        content
    );

    return response;
}