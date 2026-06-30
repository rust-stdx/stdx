use http::{
    Client,
    common::{Method, Request, Uri},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "https://example.com".to_string());

    println!("Sending request to {target}...");

    let client = Client::new();
    let req = Request::new(Method::Get, Uri::parse(&target).unwrap(), None);

    let resp = client.send(req).await?;

    println!("{} {}", resp.status.as_u16(), resp.status.canonical_reason().unwrap_or(""));
    for (name, value) in &resp.headers {
        println!("{name}: {value}");
    }
    if !resp.body.is_empty() {
        let body_str = String::from_utf8_lossy(&resp.body);
        println!("\n{body_str}");
    }

    client.close().await;
    Ok(())
}
