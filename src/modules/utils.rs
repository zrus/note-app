pub fn random_name() -> String {
  let bytes = rand::random::<[u8; 4]>();
  hex::encode(&bytes)
}
