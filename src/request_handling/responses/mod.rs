use crate::database::links::Links;

mod not_found;
mod redirect_trough_http302;
mod invalid;
mod redirect_trough_javascript;
mod internal_error;

pub fn internal_server_error() -> String { return internal_error::response(); }

/// Returns 400 Bad Request
pub fn invalid_response() -> String {
    return invalid::response();
}

/// Returns 404 not found
pub fn not_found_response() -> String {
    return not_found::response();
}

/// returns redirect trough HTTP 302 or HTTP 200 & JavaScript
pub async fn redirect_response(link: Links) -> String {
    if link.lnk_hide_target == 0 {
        return redirect_trough_http302::response(link);
    }

    return redirect_trough_javascript::response(link).await;
}