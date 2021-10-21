use crate::database::links::Links;

pub(crate) fn response(link: Links) -> String {
    let response = format!(
        "HTTP/1.1 302 Found\r\nLocation: {}\r\n\r\n",
        link.lnk_target
    );

    return response;
}