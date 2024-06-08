use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use popcorn::config::server_config;



#[get("/hello-world")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

async fn echo() -> impl Responder {
    HttpResponse::Ok().body("echo.....")
}

#[actix_web::main] 
async fn main() -> std::io::Result<()> {

    let config = server_config::Config::from_env();

    println!("Environtment configuration: {}:{}", config.port, config.host);

    const ADDRESS: &str = "127.0.0.1";
    const PORT: u16 = 6060;
    println!("Server running on http:/{}:{}/", ADDRESS, PORT);
    HttpServer::new(|| {
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
    }).bind((ADDRESS, PORT))?
    .run()
    .await
}