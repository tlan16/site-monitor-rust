use reqwest::{Error, Response};
use crate::config;
use crate::libs::notify_error::notify_error;

pub fn on_error(watch_list_item: config::watch_list::WatchListItem, error: Option<Error>, response: Option<Response>) {
    let mut message = format!("{:?} {}. Expecting http status code {}).", watch_list_item.http_method, watch_list_item.url, watch_list_item.expected_http_code);
    if let Some(response) = response {
        message.push_str(&format!(" Got http status code {}.", response.status()));
    }
    if let Some(error) = error {
        message.push_str(&format!(" Got error: {}.", error));
    }

    log::error!("{}", message);

    notify_error(format!("Failed to {} {}", watch_list_item.http_method, watch_list_item.url), message);
}