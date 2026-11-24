//! This is a pretty straightforward TODO app using the hyper HTTP library.
//! You can add TODOs to your list but only if they meet certain criteria.
//!
//! To run it:
//!
//! ```shell
//! cargo run --bin hyper --features hyper
//! ```
//!
//! Add a TODO (HTTP 202):
//!
//! ```shell
//! curl -w "%{http_code}" -XPOST -H "content-type: application/json" \
//!   http://localhost:8080/todos -d '{"kind":"work","text":"Learn more Rust"}'
//! ```
//!
//! Fetch the current TODOs:
//!
//! ```shell
//! curl http://localhost:8080/todos
//! ```
//!
//! Add another TODO (HTTP 400):
//!
//! ```shell
//! curl -w "%{http_code}" -XPOST -H "content-type: application/json" \
//!   http://localhost:8080/todos -d '{"kind":"home","text":"Learn more Rust"}'
//! ```

use std::{
    convert::Infallible,
    sync::{Arc, Mutex},
};

use bel::{Context, Program, Value};
use bytes::Bytes;
use hyper::{
    Method, Request, Response, StatusCode, body::Incoming, header::CONTENT_TYPE, server::conn::http1,
    service::service_fn,
};
use hyper_utils::{
    http_body_util::{BodyExt, Full},
    rt::TokioIo,
};
use serde::{Deserialize, Serialize};

// Policies dictating which text TODOs may contain
const WORK_TODO_POLICY: &str = r#"!text.contains("TV")"#; // Don't watch TV at work!
const HOME_TODO_POLICY: &str = r#"!text.contains("Rust")"#; // Don't hack on Rust at home!

// TODOs carry some text and have a kind
#[derive(Clone, Deserialize, Serialize)]
struct Todo {
    text: String,
    kind: TodoKind,
}

// TODOs are either for work or for home
#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum TodoKind {
    Work,
    Home,
}

// POST a new TODO to the list
async fn add_todo(ctx: AppContext, request: Request<Incoming>) -> Response<Full<Bytes>> {
    // Read the request body and decode the TODO
    let body = match request.into_body().collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(_) => return empty_response(StatusCode::BAD_REQUEST),
    };
    let todo: Todo = match serde_json::from_slice(&body) {
        Ok(todo) => todo,
        Err(_) => return empty_response(StatusCode::BAD_REQUEST),
    };

    // Use the policy decider to see if the TODO is allowed
    match ctx.decider.todo_is_allowed(&todo) {
        Ok(is_allowed) => {
            // If not, throw HTTP 400
            if !is_allowed {
                return empty_response(StatusCode::BAD_REQUEST);
            }

            // If allowed, add it to the TODOs and return HTTP 202
            let mut todos = ctx.todos.lock().unwrap();
            todos.push(todo);
            empty_response(StatusCode::ACCEPTED)
        }
        // If there's an error return an HTTP 500
        Err(error) => {
            eprintln!("policy error: {error}");
            empty_response(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// GET the current list of TODOs
fn list_todos(ctx: &AppContext) -> Response<Full<Bytes>> {
    let todos = ctx.todos.lock().unwrap();
    match serde_json::to_vec(&*todos) {
        Ok(body) => json_response(StatusCode::OK, body),
        Err(_) => empty_response(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// The policy engine for our TODOs app
struct PolicyDecider(Context<'static>);

impl PolicyDecider {
    // Start with a wrapper around the default Context
    fn new() -> Self {
        Self(Context::default())
    }

    // Determine whether a given TODO is allowed
    fn todo_is_allowed(&self, todo: &Todo) -> Result<bool, TodosError> {
        // Create a new mutable context out of the root context
        let mut ctx = self.0.new_inner_scope();
        // Add the TODO's text as a variable so that it can be part of the expression
        ctx.add_variable_from_value("text", todo.text.clone());

        // Which policy to enforce depends on the kind of TODO
        let policy = match todo.kind {
            TodoKind::Home => HOME_TODO_POLICY,
            TodoKind::Work => WORK_TODO_POLICY,
        };

        // Compile the program
        let program = Program::compile(policy)?;

        // Execute the program and either return a Boolean or the TODO is
        // considered invalid
        match program.execute(&ctx)? {
            Value::Bool(b) => Ok(b),
            _ => Err(TodosError::Invalid),
        }
    }
}

// Custom error type
#[derive(Debug, thiserror::Error)]
enum TodosError {
    #[error("CEL execution error: {0}")]
    Execution(#[from] bel::ExecutionError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("CEL parse error: {0}")]
    Parse(#[from] bel::ParseErrors),
    #[error("invalid TODO")]
    Invalid,
}

// The state shared by every connection
#[derive(Clone)]
struct AppContext {
    todos: Arc<Mutex<Vec<Todo>>>,
    decider: Arc<PolicyDecider>,
}

// Route one request to the matching handler.
async fn handle(ctx: AppContext, request: Request<Incoming>) -> Response<Full<Bytes>> {
    match (request.method(), request.uri().path()) {
        (&Method::GET, "/todos") => list_todos(&ctx),
        (&Method::POST, "/todos") => add_todo(ctx, request).await,
        (_, "/todos") => empty_response(StatusCode::METHOD_NOT_ALLOWED),
        _ => empty_response(StatusCode::NOT_FOUND),
    }
}

// Build an empty response with the given status.
fn empty_response(status: StatusCode) -> Response<Full<Bytes>> {
    Response::builder()
        .status(status)
        .body(Full::new(Bytes::new()))
        .expect("response status is valid")
}

// Build a JSON response from an already encoded body.
fn json_response(status: StatusCode, body: Vec<u8>) -> Response<Full<Bytes>> {
    Response::builder()
        .status(status)
        .header(CONTENT_TYPE, "application/json")
        .body(Full::new(Bytes::from(body)))
        .expect("response status and headers are valid")
}

#[tokio::main]
async fn main() -> Result<(), TodosError> {
    let ctx = AppContext {
        todos: Arc::new(Mutex::new(Vec::new())),
        decider: Arc::new(PolicyDecider::new()),
    };

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (stream, _) = listener.accept().await?;
        let ctx = ctx.clone();

        tokio::spawn(async move {
            let io = TokioIo::new(stream);
            let service = service_fn(move |request: Request<Incoming>| {
                let ctx = ctx.clone();
                async move { Ok::<_, Infallible>(handle(ctx, request).await) }
            });

            if let Err(error) = http1::Builder::new().serve_connection(io, service).await {
                eprintln!("connection error: {error}");
            }
        });
    }
}
