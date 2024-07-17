use axum::{body::Body, extract::{Path, Request}, http::{ Response, StatusCode, Uri}, routing::get, Router};
use popcorn::settings::config::Config;
use std::{collections::HashMap, convert::Infallible, process};
use popcorn::core::routes::RouterBucket;
use tower::service_fn;


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

    let mut router: Router = Router::new().route_service("/brand", service_fn(|req: Request| async move {
        let body = Body::from(format!("This is from route service Medthid {}", req.method()));
        let res = Response::new(body);
        Ok::<_,Infallible>(res)
    }));

    router = router.nest("/log", Router::new().route("/dev", get(|| async {
        "Dev log"
    })));

    router = router.nest("/users", Router::new().route("/", get(|uri: Uri| async move {
        format!("User routes {}", uri)
    })).fallback(user_fallback));

    router = router.nest("/public", Router::new().route("/", get(|| async {
        "Public routes"
    })).fallback(public_fallback));

    let user_api: Router = Router::new().route("/user/:id", get(users_get));

    router = router.nest("/:version/api", user_api);

    root_router.push(router);
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

// fallback 1
async fn user_fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("User not found for {}", uri))
}

// fallback 2
async fn public_fallback() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Public url not found")
}

// user handler
async fn users_get(uri: Uri, Path(params): Path<HashMap<String, String>>) -> String {
    let version = params.get("version");
    let id = params.get("id");
    println!("Version {}, id {}", version.unwrap(), id.unwrap());
    format!("Version: {}, id: {} {}", version.unwrap(), id.unwrap(), uri)
}

// not found
async fn not_found(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {uri}"))
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