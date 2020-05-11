use actix_web::{post, web, Responder};
use crate::models::blog::BlogPost;

/**
 * send_email
 * 
 * Will be filled in later, after I set up Mailgun
 */
#[post("/send_email")]
pub async fn send_email() -> impl Responder {
    let mut vec:Vec<BlogPost> = Vec::new();
    vec.push(BlogPost::new());
    return web::Json(vec);
}