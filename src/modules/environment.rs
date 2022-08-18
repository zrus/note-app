use clap::Parser;
use std::net::SocketAddr;

#[derive(Debug, Parser)]
pub struct Opts {
  #[clap(short, long)]
  bootstrap: Vec<SocketAddr>,
  #[clap(long)]
  bind: Option<SocketAddr>,
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

  pub fn command(&self) -> &Command {
    &self.command
  }
}

#[derive(Debug, Parser)]
pub enum Command {
  Bootstrap,
  Join(JoinOpts),
}

#[derive(Debug, Parser)]
pub struct JoinOpts {
  #[clap(short, long)]
  topics: Vec<String>,
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
