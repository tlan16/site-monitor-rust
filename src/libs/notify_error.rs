use std::env;
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use crate::config::environment_variables::get_environment_variables;

pub fn notify_error(subject: String, message: String) {
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
