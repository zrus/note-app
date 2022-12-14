use log::{LevelFilter, SetLoggerError};
use log4rs::{
  append::console::ConsoleAppender,
  config::{Appender, Root},
  encode::pattern::PatternEncoder,
  filter::threshold::ThresholdFilter,
  Config, Handle,
};

use super::prelude::LogLevel;

// const LOG_PATTERN: &str = "{d(%d/%m/%Y-%H:%M:%S%.6f %Z)} {h({l})} {m}{n}";
const LOG_PATTERN: &str = "[{d(%d/%m/%Y %H:%M:%S)}] {h({l})}: {m}{n}";

pub fn setup_logger(level: &LogLevel) -> Result<Handle, SetLoggerError> {
  let level = match level {
    LogLevel::TRACE => LevelFilter::Trace,
    LogLevel::DEBUG => LevelFilter::Debug,
    LogLevel::INFO => LevelFilter::Info,
    LogLevel::WARN => LevelFilter::Warn,
    LogLevel::ERROR => LevelFilter::Error,
  };

  let logger_config = Config::builder()
    .appender(
      Appender::builder()
        .filter(Box::new(ThresholdFilter::new(level)))
        .build(
          "console",
          Box::new(
            ConsoleAppender::builder()
              .encoder(Box::new(PatternEncoder::new(LOG_PATTERN)))
              .build(),
          ),
        ),
    )
    .build(
      Root::builder()
        .appender("console")
        .build(LevelFilter::Trace),
    )
    .unwrap();

  Ok(log4rs::init_config(logger_config)?)
}
