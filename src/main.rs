extern crate core;

use std::env;
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::header::{ContentType};
use lettre::transport::smtp::authentication::Credentials;
use reqwest::{Error, Response};
use tokio;
use crate::config::environment_variables::{get_environment_variables, validate_environment_variables};

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
            let client = reqwest::Client::new();
            let user_agent: String = latest_user_agent::get_latest_user_agent(
                latest_user_agent::Browser::Firefox,
                latest_user_agent::OS::WindowsIntelx64,
                false,
            ).await.user_agent;

            match client
                .get(item.url.clone())
                .header("User-Agent", user_agent)
                .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8")
                .header("Accept-Language", "en-US,en;q=0.5")
                .send()
                .await {
                Ok(response) => {
                    if response.status() == item.expected_http_code.clone() {
                        on_ok(item.clone(), response);
                    } else {
                        on_error(item.clone(), None, Some(response));
                    }
                }
                Err(error) => {
                    on_error(item.clone(), Some(error), None);
                }
            }
        });
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
}

fn on_ok(watch_list_item: config::watch_list::WatchListItem, _response: Response) {
    log::info!("{:?} {}. Got expected http status code {}.", watch_list_item.http_method, watch_list_item.url, watch_list_item.expected_http_code);
}


fn notify_error(subject: String, message: String) {
    let email = Message::builder()
        .from(format!("site-monitor-rust <{}>", env::var("APP_EMAIL_USERNAME").unwrap()).parse().unwrap())
        .reply_to("Frank Lan <franklan118@gmail.com>".parse().unwrap())
        .to(
            format!(
                "{} <{}>",
                get_environment_variables().get("APP_EMAIL_RECIPIENT_NAME").unwrap(),
                get_environment_variables().get("APP_EMAIL_RECIPIENT_EMAIL").unwrap(),
            ).parse().unwrap())
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(message)
        .unwrap();

    let creds = Credentials::new(
        get_environment_variables().get("APP_EMAIL_USERNAME").unwrap().to_owned(),
        get_environment_variables().get("APP_EMAIL_PASSWORD").unwrap().to_owned(),
    );
    let mailer = SmtpTransport::starttls_relay("smtp-mail.outlook.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => log::info!("Email sent successfully!"),
        Err(e) => log::error!("Could not send email: {e:?}"),
    }
}

fn on_error(watch_list_item: config::watch_list::WatchListItem, error: Option<Error>, response: Option<Response>) {
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
