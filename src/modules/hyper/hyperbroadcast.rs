use std::sync::Arc;

use anyhow::Result;
use async_std::channel::{Receiver, Sender};
use async_std::io::{ReadExt, WriteExt};
use async_std::stream::StreamExt;
use bastion::prelude::{spawn, JoinHandle};
use hyperswarm::{Hyperswarm, HyperswarmStream};
use tokio::select;
use tokio::sync::RwLock;

use super::{incoming_message::IncomingMessage, peer::Peer};
use crate::{debug, error, info};

#[derive(Clone)]
pub struct HyperBroadcast {
  initial_msg: Vec<u8>,
  peers: Arc<RwLock<Vec<Peer>>>,
}

impl HyperBroadcast {
  pub fn new(initial_msg: Vec<u8>) -> Self {
    Self {
      initial_msg,
      peers: Default::default(),
    }
  }

  pub async fn broadcast(&self, msg: String) {
    let peers = self.peers.read().await;
    for peer in peers.iter() {
      match peer.send_msg(&msg).await {
        Ok(_) => debug!("Message sent to {}", peer.id()),
        Err(e) => error!("Error sending message to {}: {}", peer.id(), e),
      }
    }
  }

  pub async fn run(
    &self,
    mut swarm: Hyperswarm,
  ) -> (JoinHandle<Result<()>>, Receiver<IncomingMessage>) {
    let (incoming_tx, incoming_rx) = async_std::channel::unbounded();
    let peers = self.peers.clone();
    let initial_msg = self.initial_msg.clone();
    let task = spawn(async move {
      while let Some(stream) = swarm.next().await {
        let stream = stream?;

        let (outbound_tx, outbound_rx) = async_std::channel::unbounded();
        let (inbound_tx, mut inbound_rx) = async_std::channel::unbounded();

        // Queue initial message.
        outbound_tx.send(initial_msg.clone()).await?;

        // Construct peer info.
        let peer = Peer {
          outbound_tx,
          addr: stream.peer_addr(),
          protocol: stream.protocol().to_string(),
        };
        let peer_id = peer.id();
        info!("[{peer_id}] connected");

        // Spawn loop to send and receive messages.
        {
          let peers = peers.clone();
          spawn(async move {
            if let Err(e) = connection_loop(stream, inbound_tx, outbound_rx).await {
              info!("[{peer_id}] disconnected");
              debug!("[{peer_id}] error: {e}");
              let mut peers = peers.write().await;
              peers.retain(|peer| peer.id() != peer_id);
            }
          });
        }

        // Spawn loop to forward incoming messages.
        {
          let peer_id = peer.id();
          let incoming_tx = incoming_tx.clone();
          spawn(async move {
            while let Some(msg) = inbound_rx.next().await {
              let msg = IncomingMessage {
                from: peer_id.clone(),
                content: msg,
              };
              incoming_tx.send(msg).await.unwrap();
            }
          });
        }

        // Save peer for broadcasting.
        let mut peers = peers.write().await;
        peers.push(peer);
      }
      Ok(())
    });
    (task, incoming_rx)
  }
}

async fn connection_loop(
  mut stream: HyperswarmStream,
  inbound_tx: Sender<Vec<u8>>,
  mut outbound_rx: Receiver<Vec<u8>>,
) -> Result<()> {
  let mut len_buf = [0u8; 4];
  loop {
    select! {
      // Incoming message.
      res = stream.read_exact(&mut len_buf) => match res {
          Ok(_) => {
              let len = u32::from_be_bytes(len_buf);
              let mut buf = vec![0u8; len as usize];
              stream.read_exact(&mut buf).await?;
              inbound_tx.send(buf).await?;
          }
          Err(err) => return Err(err.into()),
      },
      // Outgoing message.
      msg = outbound_rx.next() => match msg {
          Some(message) => {
              let mut buf = Vec::new();
              buf.extend((message.len() as u32).to_be_bytes());
              buf.extend(&message[..]);
              stream.write_all(&buf).await?;
          }
          None => return Err(anyhow::anyhow!("Remote connection closed?")),
      },
    }
  }
}
