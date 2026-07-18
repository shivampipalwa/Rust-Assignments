/*
  Problem 99: Async TCP Echo Server (Simplified)

  Write an async function that starts a mock TCP echo server using
  tokio::net::TcpListener on a given port. It should accept one connection,
  read exactly 5 bytes, and write them back. Return the bytes read.

  Run the tests for this problem with:
    cargo test --test echo_server_test
*/

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

pub async fn run_echo_server(port: u16) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let addr = format!("127.0.0.1:{}", port);
    let listner = TcpListener::bind(&addr).await?;

    let (mut socket, _peer_addr) = listner.accept().await?;
    let mut buf = [0u8; 10];

    socket.read(&mut buf).await?;

    socket.write(&buf).await?;

    Ok(buf.trim_ascii_end().to_vec())
}
