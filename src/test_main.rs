use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};


use popcorn::config::server_config;


#[get("/hello-world")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

async fn echo() -> impl Responder {
    HttpResponse::Ok().body("echo.....")
}

#[tokio::main] 
async fn main() -> std::io::Result<()> {

    let config = server_config::Config::from_env();


    let server = HttpServer::new(|| {
        App::new()
            .service(hello_world)
            .route("/echo", web::get().to(echo))  
            .service(
                web::scope("/api")
                .route("/", web::to(|| async { HttpResponse::Ok().body("api router") }))
            )
            .service(
                web::scope("/subscribe")
                .route("/", web::to(|| async {HttpResponse::Ok().body("subscription")}))
            )
    }).bind((config.host.clone(), config.port.clone()))?
    .run();
    
    println!("Server running on http:/{}:{}/", config.host, config.port);
    server.await

}