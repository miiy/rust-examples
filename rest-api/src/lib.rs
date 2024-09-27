use actix_web::web;
use crate::user::routes;
pub mod user;

pub fn config_app(cfg: &mut web::ServiceConfig) {
    routes::init_routes(cfg);
}