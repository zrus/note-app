use clap::Parser;
use std::{net::SocketAddr, str::FromStr};

#[derive(Debug)]
pub enum LogLevel {
  TRACE,
  DEBUG,
  INFO,
  WARN,
  ERROR,
}

impl FromStr for LogLevel {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "trace" => Ok(LogLevel::TRACE),
      "debug" => Ok(LogLevel::DEBUG),
      "info" => Ok(LogLevel::INFO),
      "warn" => Ok(LogLevel::WARN),
      "error" => Ok(LogLevel::ERROR),
      _ => Err(String::from("Unable to parse log level")),
    }
  }
}

#[derive(Debug, Parser)]
pub struct Opts {
  /// Set bootstrap addresses
  #[clap(short, long)]
  bootstrap: Vec<SocketAddr>,
  /// Address to bind DHT node to (default: 0.0.0.0:49737).
  #[clap(long)]
  bind: Option<SocketAddr>,
  /// Log level
  #[clap(short, default_value = "info")]
  level: LogLevel,
  /// Command to run
  #[clap(subcommand)]
  command: Command,
}

impl Opts {
  pub fn bootstrap(&self) -> &[SocketAddr] {
    &self.bootstrap
  }

  pub fn bind(&self) -> Option<&SocketAddr> {
    self.bind.as_ref()
  }

  pub fn log_level(&self) -> &LogLevel {
    &self.level
  }

  pub fn command(&self) -> &Command {
    &self.command
  }
}

#[derive(Debug, Parser)]
pub enum Command {
  /// Run a DHT bootstrap node.
  Bootstrap,
  /// Join the swarm and connect to peers.
  Join(JoinOpts),
}

#[derive(Debug, Parser)]
pub struct JoinOpts {
  /// Set topics
  #[clap(short, long)]
  topics: Vec<String>,
  /// Set name to send to peers in hello message
  #[clap(short, long)]
  name: Option<String>,
}

impl JoinOpts {
  pub fn topics(&self) -> &[String] {
    &self.topics
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }
}
