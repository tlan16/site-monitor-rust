use std::env;

use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

fn get_log_level() -> LevelFilter {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"-v".to_string()) {
        LevelFilter::Debug
    } else {
        LevelFilter::Debug
    }
}

pub fn init_logger() {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d} - {h({l})} {m}{n}"
        )))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(
            Root::builder()
                .appender("stdout")
                .build(get_log_level()),
        )
        .unwrap();

    log4rs::init_config(config).unwrap();
}