use actix_web::{get, HttpResponse, post, Responder, web};

pub fn init(service_config: &mut web::ServiceConfig) {
    service_config.service(
        web::scope("/hello")
            .service(hello_get)
            .service(hello_post)
    );
}

#[get("/*")]
async fn hello_get() -> impl Responder {
    HttpResponse::Ok().body("Hello from GET.")
}

#[post("/some")]
async fn hello_post() -> impl Responder {
    HttpResponse::Ok().body("Hello from POST.")
}
