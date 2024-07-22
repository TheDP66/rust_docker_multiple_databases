use actix_web::{web::Data, HttpResponse, Responder};
use serde_json::json;

use crate::{services::user_service::UserService, AppState};

pub async fn get_user_handler(data: Data<AppState>) -> impl Responder {
    let user_service = UserService::new(data.db_wkawan.clone());

    match user_service.get_user_service().await {
        Ok(result) => {
            let response = json!({
                "payload": result,
                "status": 200,
            });

            HttpResponse::Ok().json(response)
        }
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": 500.to_owned(),
            "message": e.to_string(),
        })),
    }
}
