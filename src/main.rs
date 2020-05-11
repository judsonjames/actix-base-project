use actix_web::{
    web,
    App,
    HttpResponse,
    HttpServer,
    Responder
};

// Modules Definitions
mod handlers;
mod helpers;
mod models;
mod schemas;
mod tests;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .service(web::scope("/blogs")
                .service(handlers::blogs::get_test_blog_post)
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
