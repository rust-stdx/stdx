//! JSON-RPC 2.0 types and an async server.
//!
//! This crate provides pure data types for JSON-RPC 2.0 messages, plus a
//! [`Server`] for method registration and dispatch. It is transport-agnostic
//! and performs no I/O, so the caller drives the wire.
//!
//! # Features
//!
//! - Zero-copy deferred parsing via `serde_json::value::RawValue`
//! - Generic `Server<C>` with typed async handler registration and automatic
//!   serde (de)serialization
//! - Spec-compliant batch request/response handling, including invalid batch
//!   entries
//! - Notification support
//! - Owned context: `C` is cloned once per handler invocation (wrap it in an
//!   `Arc` for cheap clones)
//!
//! # Defining typed parameters and results
//!
//! ```rust
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Deserialize)]
//! struct AddParams {
//!     a: i64,
//!     b: i64,
//! }
//!
//! #[derive(Serialize)]
//! struct AddResult {
//!     sum: i64,
//! }
//! ```
//!
//! # Creating a server
//!
//! ```rust
//! use json_rpc::{Error, Server, ServerConfig};
//!
//! #[derive(Clone)]
//! struct AppState {
//!     prefix: String,
//! }
//!
//! # use serde::{Deserialize, Serialize};
//! # #[derive(Deserialize)]
//! # struct AddParams {
//! #     a: i64,
//! #     b: i64,
//! # }
//! # #[derive(Serialize)]
//! # struct AddResult {
//! #     sum: i64,
//! # }
//! let mut server: Server<AppState> = Server::new(ServerConfig::default());
//!
//! server.register("add", |ctx: AppState, params: AddParams| async move {
//!     Ok::<_, Error>(AddResult {
//!         sum: params.a + params.b,
//!     })
//! });
//! ```
//!
//! # Handling requests
//!
//! A single request:
//!
//! ```rust
//! # use json_rpc::{Error, Request, RequestMessage, Server, ServerConfig};
//! # #[tokio::main]
//! # async fn main() {
//! # let mut server = Server::<()>::new(ServerConfig::default());
//! # server.register("add", |_: (), (a, b): (i64, i64)| async move { Ok::<_, Error>(a + b) });
//! let json = r#"{"jsonrpc":"2.0","method":"add","params":[3,4],"id":1}"#;
//! let request: Request = serde_json::from_str(json).unwrap();
//!
//! let response = server.handle((), RequestMessage::Single(request)).await;
//! if let Some(response) = response {
//!     println!("{}", serde_json::to_string(&response).unwrap());
//!     // => {"jsonrpc":"2.0","result":7,"id":1}
//! }
//! # }
//! ```
//!
//! A notification has no `id`, so [`Server::handle`] returns `None` and nothing
//! should be sent back:
//!
//! ```rust
//! # use json_rpc::{Error, Request, RequestMessage, Server, ServerConfig};
//! # #[tokio::main]
//! # async fn main() {
//! # let mut server = Server::<()>::new(ServerConfig::default());
//! # server.register("log", |_: (), _message: String| async move { Ok::<_, Error>(()) });
//! let json = r#"{"jsonrpc":"2.0","method":"log","params":"hello"}"#;
//! let request: Request = serde_json::from_str(json).unwrap();
//!
//! let response = server.handle((), RequestMessage::Single(request)).await;
//! assert!(response.is_none());
//! # }
//! ```
//!
//! A batch:
//!
//! ```rust
//! # use json_rpc::{Error, RequestMessage, Server, ServerConfig};
//! # #[tokio::main]
//! # async fn main() {
//! # let mut server = Server::<()>::new(ServerConfig::default());
//! # server.register("add", |_: (), (a, b): (i64, i64)| async move { Ok::<_, Error>(a + b) });
//! let json = r#"[
//!     {"jsonrpc":"2.0","method":"add","params":[1,2],"id":"1"},
//!     {"jsonrpc":"2.0","method":"add","params":[3,4],"id":"2"}
//! ]"#;
//! let message: RequestMessage = serde_json::from_str(json).unwrap();
//!
//! let response = server.handle((), message).await;
//! if let Some(response) = response {
//!     println!("{}", serde_json::to_string(&response).unwrap());
//! }
//! # }
//! ```
//!
//! # Custom errors
//!
//! A handler may return any error type that converts into [`Error`]:
//!
//! ```rust
//! # use json_rpc::{Error, Server, ServerConfig};
//! # let mut server = Server::<()>::new(ServerConfig::default());
//! #[derive(Debug)]
//! enum AppError {
//!     DivisionByZero,
//!     Db(String),
//! }
//!
//! impl From<AppError> for Error {
//!     fn from(e: AppError) -> Self {
//!         match e {
//!             AppError::DivisionByZero => Error::new(-32000, "division by zero"),
//!             AppError::Db(message) => Error::new(-32001, format!("internal: {message}")),
//!         }
//!     }
//! }
//!
//! server.register("div", |_: (), (a, b): (i64, i64)| async move {
//!     if b == 0 {
//!         Err(AppError::DivisionByZero)
//!     } else {
//!         Ok(a / b)
//!     }
//! });
//! ```
//!
//! # Serving over a transport
//!
//! The server is transport-agnostic. For example, over a newline-delimited TCP
//! stream:
//!
//! ```no_run
//! use json_rpc::{Error, Id, RequestMessage, Response, Server, ServerConfig};
//! use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
//! use tokio::net::TcpListener;
//!
//! #[tokio::main]
//! async fn main() -> std::io::Result<()> {
//!     let mut server = Server::<()>::new(ServerConfig::default());
//!     server.register("ping", |_: (), ()| async move { Ok::<_, Error>("pong".to_string()) });
//!
//!     let listener = TcpListener::bind("127.0.0.1:8080").await?;
//!     loop {
//!         let (stream, _) = listener.accept().await?;
//!         let (reader, mut writer) = stream.into_split();
//!         let mut lines = BufReader::new(reader).lines();
//!
//!         while let Ok(Some(line)) = lines.next_line().await {
//!             let message: RequestMessage = match serde_json::from_str(&line) {
//!                 Ok(message) => message,
//!                 Err(_) => {
//!                     let error = Response::error(Id::Null, Error::parse_error());
//!                     writer.write_all(serde_json::to_string(&error).unwrap().as_bytes()).await?;
//!                     writer.write_all(b"\n").await?;
//!                     continue;
//!                 }
//!             };
//!
//!             if let Some(response) = server.handle((), message).await {
//!                 writer.write_all(serde_json::to_string(&response).unwrap().as_bytes()).await?;
//!                 writer.write_all(b"\n").await?;
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! Over HTTP, deserialize a [`RequestMessage`] from the request body, call
//! [`Server::handle`], and serialize the returned [`ResponseMessage`] (or reply
//! `204 No Content` when it is `None`):
//!
//! ```ignore
//! use std::sync::Arc;
//!
//! use axum::{
//!     extract::State,
//!     extract::rejection::JsonRejection,
//!     http::StatusCode,
//!     response::IntoResponse,
//!     routing::post,
//!     Json, Router,
//! };
//! use json_rpc::{Error, Id, RequestMessage, Response, Server, ServerConfig};
//!
//! async fn rpc_handler(
//!     State(server): State<Arc<Server<AppState>>>,
//!     State(state): State<Arc<AppState>>,
//!     payload: Result<Json<RequestMessage>, JsonRejection>,
//! ) -> impl IntoResponse {
//!     let message = match payload {
//!         Ok(Json(message)) => message,
//!         Err(_) => {
//!             let error = Response::error(Id::Null, Error::parse_error());
//!             return (StatusCode::OK, Json(error)).into_response();
//!         }
//!     };
//!
//!     match server.handle((*state).clone(), message).await {
//!         Some(response) => (StatusCode::OK, Json(response)).into_response(),
//!         None => StatusCode::NO_CONTENT.into_response(),
//!     }
//! }
//! ```

pub mod error_code;
pub mod id;
pub mod request;
pub mod response;
pub mod server;

pub use error_code::ErrorCode;
pub use id::Id;
pub use request::{Request, RequestId, RequestMessage};
pub use response::{Error, Response};
pub use server::{ResponseMessage, Server, ServerConfig};
