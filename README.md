# **NOTE APP USING HYPERCORE ECOSYSTEM**

*NOTE: This is still in early stages. See the roadmap below. Please feel free to open issues and send PRs :-)*

Hyperswarm is a networking stack for connecting peers who are interested in a topic. This project is a port of the [Node.js implementation of Hyperswarm](https://github.com/hyperswarm/hyperswarm).

This crate exposes a `Hyperswarm` struct. After binding it, this will:

- Start and bootstrap a local DHT node
- Bind a socket for mDNS discovery
- Announce and lookup any 32 byte topic key over both mDNS and the DHT
- Connect to all peers that are found over both TCP and UTP

It currently depends on the unreleased [hyperswarm-dht](https://github.com/datrs/hyperswarm-dht) crate and therefore is also not yet released on crates.io.

The API is designed to be very simple:

```rust
use async_std::task;
use futures_lite::{AsyncReadExt, AsyncWriteExt, StreamExt};
use hyperswarm::{Config, Hyperswarm, HyperswarmStream, TopicConfig};
use std::io;

#[async_std::main]
async fn main() -> io::Result<()> {
    // Bind and initialize the swarm with the default config.
    // On the config you can e.g. set bootstrap addresses.
    let config = Config::default();
    let mut swarm = Hyperswarm::bind(config).await?;

    // A topic is any 32 byte array. Usually, this would be the hash of some identifier.
    // Configuring the swarm for a topic starts to lookup and/or announce this topic
    // and connect to peers that are found.
    let topic = [0u8; 32];
    swarm.configure(topic, TopicConfig::announce_and_lookup());

    // The swarm is a Stream of new HyperswarmStream peer connections.
    // The HyperswarmStream is a wrapper around either a TcpStream or a UtpSocket.
    // Usually you'll want to run some loop over the connection, so let's spawn a task
    // for each connection.
    while let Some(stream) = swarm.next().await {
        task::spawn(on_connection(stream?));
    }

    Ok(())
}

// A HyperswarmStream is AsyncRead + AsyncWrite, so you can use it just
// like a TcpStream. Here, we'll send an initial message and then keep
// reading from the stream until it is closed by the remote.
async fn on_connection(mut stream: HyperswarmStream) -> io::Result<()> {
    stream.write_all(b"hello there").await?;
    let mut buf = vec![0u8; 64];
    loop {
        match stream.read(&mut buf).await {
            Ok(0) => return Ok(()),
            Err(e) => return Err(e),
            Ok(n) => eprintln!("received: {}", std::str::from_utf8(&buf[..n]).unwrap()),
        }
    }
}
```

> Currently, this is a demo for chatting between peers. Upcoming we will upgrade it to a decentralized notebook for peers.

## **USAGE**

### **Hyperchat**

This is a very basic chat-over-hyperswarm demo. To try locally you will need three terminals.
1. Run a bootstrap node:
```bash
$ cargo run -- bootstrap
Running bootstrap node on 0.0.0.0:49737
```
2. Run clients and chat with each other:
```bash
$ cargo run -- join --topics asdf --name alice
your name: alice
join topic "asdf": a05d11c6234b3321315ec175592dfc193f5650a28b569b3e09bac5a4216bb138
[tcp:127.0.0.1:42172] connected
[utp:127.0.0.1:38683] connected
[utp:127.0.0.1:38683] is now known as `bob`
[tcp:127.0.0.1:42172] disconnected
[utp:127.0.0.1:38683] <bob> hi there :)
hello!
```
```bash
$ cargo run -- join -t asdf -n bob
your name: bob
join topic "asdf": a05d11c6234b3321315ec175592dfc193f5650a28b569b3e09bac5a4216bb138
[utp:127.0.0.1:41187] connected
[utp:127.0.0.1:41187] is now known as `alice`
hi there :)
[utp:127.0.0.1:41187] <alice> hello!
```
That's all. Have fun with the demo. Any issue is welcome.