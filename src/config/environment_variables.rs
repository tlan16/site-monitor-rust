use dotenvy::dotenv;
use std::collections::HashMap;
use std::env;
use validators::prelude::*;
use validators::models::Host;

#[derive(Validator)]
#[validator(text(char_length(trimmed_min = 1, min = 1, max = 250)))]
struct TextNotAllowEmpty(pub String);

#[derive(Validator)]
#[validator(email(comment(Disallow), ip(Allow), local(Disallow), at_least_two_labels(Allow), non_ascii(Allow)))]
pub struct EmailWithoutComment {
    pub local_part: String,
    #[allow(dead_code)]
    pub need_quoted: bool,
    pub domain_part: Host,
}

#[derive(Validator)]
#[validator(host(local(Allow), port(Disallow), at_least_two_labels(Must)))]
pub struct HostMustAtLeastTwoLabelsAllowPort {
    pub host: Host,
    #[allow(dead_code)]
    pub is_local: bool,
}

pub fn get_environment_variables() -> HashMap<String, String> {
    dotenv().expect(".env file not found");
    let mut variables = HashMap::new();
    for (key, value) in env::vars() {
        variables.insert(key, value);
    }
    variables
}

pub fn validate_environment_variables() {
    assert!(EmailWithoutComment::parse_string(get_environment_variables().get("APP_EMAIL_USERNAME").unwrap()).is_ok());
    assert!(TextNotAllowEmpty::parse_string(get_environment_variables().get("APP_EMAIL_PASSWORD").unwrap()).is_ok());
    assert!(HostMustAtLeastTwoLabelsAllowPort::parse_string(get_environment_variables().get("APP_EMAIL_SMTP_STARTTLS_SERVER_URI").unwrap()).is_ok());
    assert!(TextNotAllowEmpty::parse_string(get_environment_variables().get("APP_EMAIL_RECIPIENT_NAME").unwrap()).is_ok());
    assert!(EmailWithoutComment::parse_string(get_environment_variables().get("APP_EMAIL_RECIPIENT_EMAIL").unwrap()).is_ok());
}
