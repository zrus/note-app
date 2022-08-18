use log::{LevelFilter, SetLoggerError};
use log4rs::{
  append::console::ConsoleAppender,
  config::{Appender, Root},
  encode::pattern::PatternEncoder,
  filter::threshold::ThresholdFilter,
  Config, Handle,
};

// const LOG_PATTERN: &str = "{d(%d/%m/%Y-%H:%M:%S%.6f %Z)} {h({l})} {m}{n}";
const LOG_PATTERN: &str = "[{d(%d/%m/%Y %H:%M:%S)}] {h({l})}: {m}{n}";

pub fn setup_logger() -> Result<Handle, SetLoggerError> {
  let logger_config = Config::builder()
    .appender(
      Appender::builder()
        .filter(Box::new(ThresholdFilter::new(LevelFilter::Debug)))
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
