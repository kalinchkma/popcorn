use axum::{routing::get, Router};
use popcorn::routes::root_router::RootRouter;
use popcorn::settings::config::Config;
use std::process;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // server configuration
    let config = Config::from_env();

    // root router
    let mut root_router = RootRouter::new();

    root_router.add(Router::new().route("/", get(root)));

    root_router.add(Router::new().route("/to", get(root)));

    // build application
    // let app = Router::new().merge(root_router.routers());
    let app = Router::new().route("/", get(root));
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

// basic handler that response with static string
async fn root() -> &'static str {
    "Hello world"
}
