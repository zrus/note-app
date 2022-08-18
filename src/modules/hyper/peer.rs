use std::net::SocketAddr;

use anyhow::Result;
use async_std::channel::Sender;

pub type PeerId = String;

pub struct Peer {
  pub outbound_tx: Sender<Vec<u8>>,
  pub addr: SocketAddr,
  pub protocol: String,
}

impl Peer {
  pub fn id(&self) -> PeerId {
    format!("{}:{}", self.protocol, self.addr)
  }

  pub async fn send_msg(&self, msg: &str) -> Result<()> {
    let msg = msg.as_bytes().to_vec();
    self.outbound_tx.send(msg).await?;
    Ok(())
  }
}
