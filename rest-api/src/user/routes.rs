use actix_web::web;
use crate::user::handler::find;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find);
}