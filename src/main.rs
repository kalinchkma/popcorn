use axum::{http::{StatusCode, Uri}, routing::get, Router};
use popcorn::settings::config::Config;
use std::process;
use popcorn::core::routes::RouterBucket;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // server configuration
    let config = Config::from_env();

    // root router
    let mut root_router = RouterBucket::new();

    let mut multi_router: RouterBucket = RouterBucket::new();

    multi_router.push(Router::new().route("/mutiple", get(root)));

    root_router.push(multi_router.combine_routers());

    root_router.push(Router::new().route("/", get(root)));

    root_router.push(Router::new().route("/to", get(new_api)));

    root_router.push(Router::new().route("/test", get(testing)));

    // build application
    let app = Router::new().merge(root_router.combine_routers()).fallback(not_found);

    // run the application
    let listener = tokio::net::TcpListener::bind(config.address())
        .await
        .unwrap_or_else(|e| {
            println!("Error: {:?}", e);
            process::exit(0)
        });

    println!("Server running on {:?}", config.address());
    // serving apps
    axum::serve(listener, app).await.unwrap_or_else(|e| {
        println!("Error: {:?}", e);
        process::exit(0)
    });
}

// not found
async fn not_found(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route fot {uri}"))
}

// basic handler that response with static string
async fn root() -> &'static str {
    "Hello world"
}

async fn new_api() -> &'static str {
    "Combine router is working"
}

async fn testing() -> &'static str {
    "Testing router"
}