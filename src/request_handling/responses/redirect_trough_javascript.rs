use std::ops::Add;
use std::str;
use log::{info, trace, warn};
use rbatis::crud::CRUD;
use crate::database::links::Links;
use crate::database::open_graph::OpenGraph;

pub async fn response(link: Links) -> String {
    info!("Creating redirect trough javascript");

    let tags : Vec<OpenGraph> = crate::database::RB.fetch_list_by_column("log_link_id", &[link.lnk_id]).await.unwrap();


    let mut content: String = String::from(r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">"#);

    for og in &tags {
        let tag = create_og_tag(og);
        content += &*tag;
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
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        content.len(),
        content
    );

    return response;
}

fn create_og_tag(opengraph : &OpenGraph) -> String {
    return format!("<meta property=\"{}\" content=\"{}\" />", opengraph.log_tag, opengraph.log_content);
}