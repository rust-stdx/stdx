use std::net::ToSocketAddrs;

use http::{
    common::{Method, Request, Uri, Version},
    http1,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = std::env::args().nth(1).unwrap_or_else(|| "example.com".to_string());
    let addr = format!("{}:80", host).to_socket_addrs()?.next().unwrap();
    println!("Connecting to {} at {}...", host, addr);

    let mut stream = TcpStream::connect(addr).await?;
    println!("Connected");

    // Build the request
    let req: Request<bytes::Bytes> = Request {
        method: Method::Get,
        uri: Uri::parse("/").unwrap(),
        version: Version::Http11,
        headers: vec![
            ("host".into(), host.clone().into()),
            ("user-agent".into(), "stdx-http/0.1".into()),
            ("accept".into(), "*/*".into()),
            ("connection".into(), "close".into()),
        ],
        body: None,
    };

    // Streaming encoder — writes the request one chunk at a time
    let mut encoder = http1::encoder(req);
    let mut out = [0u8; 4096];
    while let Some(n) = encoder.encode(&mut out).await? {
        stream.write_all(&out[..n]).await?;
    }

    println!("Request sent, reading response...");

    // Streaming decoder — feeds bytes as they arrive from the socket
    let mut decoder = http1::ResponseDecoder::new();
    let mut recv_buf = vec![0u8; 65536];

    loop {
        let n = stream.read(&mut recv_buf).await?;
        if n == 0 {
            break; // connection closed
        }
        match decoder.feed(&recv_buf[..n])? {
            Some((resp, _remaining)) => {
                println!("{} {}", resp.status.as_u16(), resp.status.canonical_reason().unwrap_or(""));
                for (name, value) in &resp.headers {
                    println!("{}: {}", name, value);
                }
                if !resp.body.is_empty() {
                    println!("\n{}", String::from_utf8_lossy(&resp.body));
                }
                return Ok(());
            }
            None => {
                // incomplete — keep reading
            }
        }
    }

    eprintln!("Connection closed before full response received");
    Ok(())
}
