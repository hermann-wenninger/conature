use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there! it is conature")
}
async fn make_res() -> impl Responder {
    HttpResponse::Ok().body("Hey it's make res")
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/yx<", web::get().to(HttpResponse::Ok))).workers(4);
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .route("/make", web::get().to(make_res))
    })
   
    .bind(("127.0.0.1", 8055))?
    .run()
    .await
}
