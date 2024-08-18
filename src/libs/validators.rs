use validators::prelude::*;
use validators::models::Host;
use validators::Validator;

#[derive(Validator)]
#[validator(text(char_length(trimmed_min = 1, min = 1, max = 250)))]
pub struct TextNotAllowEmpty(pub String);

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
