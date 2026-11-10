use json_rpc::{Error, RequestMessage, Server, ServerConfig};

#[tokio::main]
async fn main() {
    let mut server: Server<()> = Server::new(ServerConfig::default());

    server.register("add", async move |_: (), (a, b): (i64, i64)| Ok::<_, Error>(a + b));

    server.register("subtract", async move |_: (), (a, b): (i64, i64)| Ok::<_, Error>(a - b));

    server.register("multiply", |_: (), (a, b): (i64, i64)| async move { Ok::<_, Error>(a * b) });

    server.register("divide", divide);

    // --- Single requests ---

    let call = |method, params| {
        json::from_str(&json::json!({"jsonrpc":"2.0","method":method,"params":params,"id":1}).to_string()).unwrap()
    };

    let resp = server.handle((), RequestMessage::Single(call("add", [3, 4]))).await;
    println!("add(3, 4)  => {}", json_from_resp(&resp));

    let resp = server
        .handle((), RequestMessage::Single(call("subtract", [10, 3])))
        .await;
    println!("sub(10, 3) => {}", json_from_resp(&resp));

    let resp = server
        .handle((), RequestMessage::Single(call("multiply", [6, 7])))
        .await;
    println!("mul(6, 7)  => {}", json_from_resp(&resp));

    let resp = server.handle((), RequestMessage::Single(call("divide", [42, 6]))).await;
    println!("div(42, 6) => {}", json_from_resp(&resp));

    let resp = server.handle((), RequestMessage::Single(call("divide", [1, 0]))).await;
    println!("div(1, 0)  => {}", json_from_resp(&resp));

    // --- Notification (no response expected) ---

    let notif = json::from_str(r#"{"jsonrpc":"2.0","method":"add","params":[1,1]}"#).unwrap();
    let resp = server.handle((), RequestMessage::Single(notif)).await;
    println!("notif      => {:?}", resp); // None

    // --- Method not found ---

    let resp = server
        .handle(
            (),
            RequestMessage::Single(json::from_str(r#"{"jsonrpc":"2.0","method":"unknown","id":1}"#).unwrap()),
        )
        .await;
    println!("unknown    => {}", json_from_resp(&resp));

    // --- Batch ---

    let batch_json = r#"[
        {"jsonrpc":"2.0","method":"add","params":[1,2],"id":"a"},
        {"jsonrpc":"2.0","method":"multiply","params":[3,4],"id":"b"},
        {"jsonrpc":"2.0","method":"subtract","params":[10,7],"id":"c"}
    ]"#;
    let message: RequestMessage = json::from_str(batch_json).unwrap();
    let resp = server.handle((), message).await;
    println!("batch      => {}", json_from_resp(&resp));
}

fn json_from_resp(message: &Option<json_rpc::ResponseMessage>) -> String {
    match message {
        Some(m) => json::to_string(m),
        None => "nothing".to_string(),
    }
}

async fn divide(_: (), (a, b): (i64, i64)) -> Result<i64, Error> {
    if b == 0 {
        Err(Error::new(-32000, "division by zero"))
    } else {
        Ok(a / b)
    }
}
