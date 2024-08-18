use std::fmt;
use std::iter::Iterator;

#[derive(Debug, Clone)]
pub enum SupportedHttpMethod {
    GET,
}
impl fmt::Display for SupportedHttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct WatchListItem {
    pub url: String,
    pub http_method: SupportedHttpMethod,
    pub expected_http_code: u16,
}

const DFCO2_URLS: [&str; 2] = [
    "https://dfco2.org.au/",
    "https://dfco2.org.au/wp-admin/",
];

const FRANK_URLS: [&str; 8] = [
    "https://auth.franklan.com.au/",
    "https://book.franklan.com.au/",
    "https://camera.franklan.com.au/",
    "https://httpbin.franklan.com.au/",
    "https://mail.franklan.com.au/",
    "https://torrent.franklan.com.au/",
    "https://traefik.franklan.com.au/",
    "https://vault.franklan.com.au/",
];

fn url_to_watch_list_item(url: &str) -> WatchListItem {
    WatchListItem {
        url: url.to_string(),
        http_method: SupportedHttpMethod::GET,
        expected_http_code: 200,
    }
}

pub fn get_watch_list() -> Vec<WatchListItem> {
    let empty_string_vec: Vec<&str> = Vec::new();
    let combined_urls: Vec<&str> = empty_string_vec
        .iter()
        .chain(DFCO2_URLS.iter())
        .chain(FRANK_URLS.iter())
        .cloned()
        .collect();
    combined_urls.into_iter()
        .map(
            |url| url_to_watch_list_item(url)
        )
        .collect()
}
