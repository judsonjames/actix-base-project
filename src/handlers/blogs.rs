use actix_web::{get, web, Responder};
use crate::models::blog::BlogPost;

#[get("/test_blogs")]
pub async fn get_test_blog_post() -> impl Responder {
    let mut vec:Vec<BlogPost> = Vec::new();
    vec.push(BlogPost::new());
    return web::Json(vec);
}