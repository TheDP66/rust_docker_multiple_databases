use actix_web::web;

use crate::handlers::user_handler::get_user_handler;

pub fn user_config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/user").route("", web::get().to(get_user_handler));

    conf.service(scope);
}
