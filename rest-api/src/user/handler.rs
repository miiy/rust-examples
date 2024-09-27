use actix_web::{get, Responder, HttpResponse};
// use crate::user::model::User;

#[get("/users/{id}")]
async fn find() -> impl Responder {
    HttpResponse::Ok().body("user")
    // HttpResponse::Ok().json(
    //     User { id: 1, email: "test@test.test".to_string()}
    // )
}
