use crate::libs::validators::{EmailWithoutComment, HostMustAtLeastTwoLabelsAllowPort, TextNotAllowEmpty};
use dotenvy::dotenv;
use std::collections::HashMap;
use std::env;
use validators::traits::ValidateString;

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
