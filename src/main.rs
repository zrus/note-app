mod modules;

use anyhow::Result;
use async_std::stream::StreamExt;
use bastion::prelude::{spawn, Bastion};
use clap::Parser;
use hyperswarm::{hash_topic, BootstrapNode, Config, DhtConfig, Hyperswarm, TopicConfig};
use modules::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
  let opts: Opts = Opts::parse();
  let _ = setup_logger();

  Bastion::init();
  Bastion::start();

  // Code goes here..
  match opts.command() {
    Command::Bootstrap => {
      let config = DhtConfig::default();
      let config = if !opts.bootstrap().is_empty() {
        config.set_bootstrap_nodes(opts.bootstrap())
      } else {
        config.empty_bootstrap_nodes()
      };
      let node = BootstrapNode::new(config, opts.bind().cloned());
      let (addr, task) = node.run().await?;
      info!("Running bootstrap node on: {:?}", addr);
      task.await?;
    }
    Command::Join(join_opts) => {
      let config = Config::default();
      let config = if !opts.bootstrap().is_empty() {
        config.set_bootstrap_nodes(opts.bootstrap())
      } else {
        config.set_bootstrap_nodes(&["127.0.0.1:49737"])
      };
      let random_name = &random_name();
      let name = join_opts.name().unwrap_or_else(|| random_name);

      // Bind and open swarm.
      let swarm = Hyperswarm::bind(config).await?;
      // Configure swarm and listen on topics.
      let handle = swarm.handle();
      let config = TopicConfig::announce_and_lookup();
      for topic in join_opts.topics() {
        let hash = hash_topic(DISCOVERY_NS_BUF, topic.as_bytes());
        handle.configure(hash, config.clone());
        info!("Joined topic \"{}\": {}", topic, hex::encode(hash));
      }

      // Open broadcaster.
      let initial_msg = name.as_bytes().to_vec();
      let broadcaster = HyperBroadcast::new(initial_msg);

      // Start the broadcast loops.
      let (task, mut incoming_rx) = broadcaster.run(swarm).await;

      // Print incoming messages.
      spawn(async move {
        let mut name = None;
        while let Some(msg) = incoming_rx.next().await {
          let content =
            String::from_utf8(msg.content).unwrap_or_else(|_| "<invalid utf8>".to_owned());
          match name.as_ref() {
            None => {
              info!("[{}] is now known as \"{content}\"", msg.from);
              name = Some(content);
            }
            Some(name) => {
              info!("[{}] <{}> {}", msg.from, name, content);
            }
          }
        }
      });

      // Read outgoing message from stdin
      {
        let broadcaster = broadcaster.clone();
        spawn(async move {
          let stdin = async_std::io::stdin();
          loop {
            let mut line = String::new();
            stdin.read_line(&mut line).await.unwrap();
            broadcaster.broadcast(line).await;
          }
        });
      }

      // Wait for the broadcast task until error or forever.
      task.await?;
    }
  }

  Bastion::block_until_stopped();
  Ok(())
}
