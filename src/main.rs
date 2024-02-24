use actix_web::{ guard,get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

async fn index_counter(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {counter}") // <- response with count
}
// This struct represents state
struct AppState {
    app_name: String,
}

#[get("/appstate")]
async fn index_state(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}!") // <- response with app_name
}

async fn index() -> impl Responder {
    "Hello world! ahaoiii 33441"
}

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
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });
    HttpServer::new(|| App::new().route("/yx<", web::get().to(HttpResponse::Ok))).workers(4);
    HttpServer::new(move|| {
        App::new()
        .app_data(counter.clone()) // <- register the created data
            .route("/index_counter", web::get().to(index_counter))

        .app_data(web::Data::new(AppState {
            app_name: String::from("Actix Web"),
        }))
            .service(index_state)
            .service(hello)
            .service(echo)

            .service(web::scope("/app")
                    .route("/index.html", web::get().to(index))
                    .route("/make", web::get().to(make_res)))

            .service(web::resource("/user/{name}")
                    .name("user_detail")
                    .guard(guard::Header("content-type", "application/json"))
                    .route(web::get().to(HttpResponse::Ok))
                    .route(web::put().to(HttpResponse::Ok)),)

            .route("/hey", web::get().to(manual_hello))
            .route("/make", web::get().to(make_res))

            .service(
                web::scope("/rust")
                    .guard(guard::Host("www.rust-lang.org"))
                    .route("rust", web::to(|| async { HttpResponse::Ok().body("www") })),
            )
            .service(
                web::scope("/rusta")
                    .guard(guard::Host("users.rust-lang.org"))
                    .route("rusta", web::to(|| async { HttpResponse::Ok().body("user") })),
            )
            .route("/rustc", web::to(HttpResponse::Ok))
            
    })
  
   
    .bind(("127.0.0.1", 8055))?
    .run()
    .await
}
