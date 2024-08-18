extern crate core;

use crate::config::environment_variables::validate_environment_variables;
use tokio;
use libs::{on_error, on_ok};
use crate::libs::http::{build_headers};

mod config;
mod libs;

#[tokio::main]
async fn main() {
    libs::logger::init_logger();
    validate_environment_variables();
    log::info!("Started at {}", chrono::Utc::now());

    let watch_list = config::watch_list::get_watch_list();

    for item in watch_list.iter().cloned() {
        tokio::spawn(async move {
            match reqwest::Client::new()
                .get(item.url.clone())
                .headers(build_headers().await)
                .send()
                .await {
                Ok(response) => {
                    if response.status() == item.expected_http_code.clone() {
                        on_ok::on_ok(item.clone(), response);
                    } else {
                        on_error::on_error(item.clone(), None, Some(response));
                    }
                }
                Err(error) => {
                    on_error::on_error(item.clone(), Some(error), None);
                }
            }
        });
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
}
