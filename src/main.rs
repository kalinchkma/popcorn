use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};


#[get("/hello-world")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

async fn echo() -> impl Responder {
    HttpResponse::Ok().body("echo.....")
}

#[actix_web::main] 
async fn main() -> std::io::Result<()> {
    const ADDRESS: &str = "127.0.0.1";
    const PORT: u16 = 6060;
    println!("Server running on http:/{}:{}/", ADDRESS, PORT);
    HttpServer::new(|| {
        App::new()
            .service(hello_world)
            .route("/echo", web::get().to(echo))  
    }).bind((ADDRESS, PORT))?
    .run()
    .await
}