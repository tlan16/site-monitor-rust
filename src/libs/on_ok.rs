use reqwest::Response;
use crate::config;

pub fn on_ok(watch_list_item: config::watch_list::WatchListItem, _response: Response) {
    log::info!("{:?} {}. Got expected http status code {}.", watch_list_item.http_method, watch_list_item.url, watch_list_item.expected_http_code);
}