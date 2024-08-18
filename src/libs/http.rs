use latest_user_agent::get_latest_user_agent;
use reqwest::header::HeaderMap;

async fn get_user_agent() -> String {
    get_latest_user_agent(
        latest_user_agent::Browser::Firefox,
        latest_user_agent::OS::WindowsIntelx64,
        false,
    ).await.user_agent
}

pub async fn build_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", get_user_agent().await.parse().unwrap());
    headers.insert("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8".parse().unwrap());
    headers.insert("Accept-Language", "en-US,en;q=0.5".parse().unwrap());
    headers
}
