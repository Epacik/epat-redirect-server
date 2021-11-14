use crate::database::links::Links;
use crate::database::open_graph::OpenGraph;
use rbatis::crud::CRUD;

// pub fn internal_server_error() -> String {
//     log::info!("Creating 500 Internal server error");
//
//     let content = r#"<!DOCTYPE html>
// <html lang="en">
//   <head>
//     <meta charset="utf-8">
//     <title>Internal server error/title>
//   </head>
//   <body>
//     <h1>Internal server error</h1>
//   </body>
// </html>"#;
//
//     let response = format!(
//         "HTTP/1.1 500 Internal Server Error\r\nContent-Length: {}\r\n\r\n{}",
//         content.len(),
//         content
//     );
//
//     return response;
// }

/// Returns 400 Bad Request
pub fn invalid_response() -> String {
    log::info!("Creating 400 bad request");

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

/// Returns 404 not found
pub fn not_found_response() -> String {
    log::info!("Creating 404 not found response");

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

/// returns redirect trough HTTP 302 or HTTP 200 & JavaScript
pub async fn redirect_response(link: Links) -> String {
    if link.lnk_hide_target == 0 {
        return redirect_trough_http302(link);
    }

    return redirect_trough_javascript(link).await;
}


pub(crate) fn redirect_trough_http302(link: Links) -> String {
    let response = format!(
        "HTTP/1.1 302 Found\r\nLocation: {}\r\n\r\n",
        link.lnk_target
    );

    return response;
}


pub async fn redirect_trough_javascript(link: Links) -> String {
    log::info!("Creating redirect trough javascript");

    let tags : Vec<OpenGraph> = crate::database::RB.fetch_list_by_column("log_link_id", &[link.lnk_id]).await.unwrap();


    let mut content: String = String::from(r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Some title</title>
    <meta name="description" content="Some description">"#);

    for og in &tags {
        let tag = create_og_tag(og);
        content += &*tag;
        content += "\n";
    }

    content += r#"
  </head>
  <body>
    <script>
    "#;

    content += &*format!(" setTimeout( x => location = \"{}\", 500);", link.lnk_target);
    content += r#"
    </script>
  </body>
</html>
    "#;

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\ncontent-type: text/html\r\n\r\n{}",
        content.len(),
        content
    );

    return response;
}

fn create_og_tag(opengraph : &OpenGraph) -> String {
    return format!("<meta property=\"{}\" content=\"{}\" />", opengraph.log_tag, opengraph.log_content);
}