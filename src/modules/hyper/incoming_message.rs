use super::peer::PeerId;

pub struct IncomingMessage {
  pub from: PeerId,
  pub content: Vec<u8>,
}
