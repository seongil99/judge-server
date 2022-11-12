use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write, net::SocketAddr};

#[derive(Deserialize, Serialize)]
struct JudgeRequest {
    code: String,
    lang: String,
}

struct JudgeQueue {
    queue: Vec<JudgeRequest>,
}

#[tokio::main]
pub async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/judge", post(judge));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn judge(Json(req): Json<JudgeRequest>) -> impl IntoResponse {
    println!("code: {}", req.code);
    println!("lang: {}", req.lang);

    let code = req.code.to_string();

    let mut code_file = File::create("main.c").unwrap();
    let buf = code.as_bytes();
    code_file.write_all(buf).unwrap();

    StatusCode::OK
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
