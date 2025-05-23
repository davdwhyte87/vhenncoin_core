use std::sync::Arc;
use actix_web::{post, web, HttpResponse};
use actix_web::web::{Data, ReqData};
use sled::Db;
use crate::models::request::TransferReq;
use crate::models::response::GenericResp;

#[post("/samp")]
pub async fn sample_controller(
    db:Data<Arc<Db>>,
    req: Result<web::Json<TransferReq>, actix_web::Error>,
)->HttpResponse {
    let mut resp_data = GenericResp::<String> {
        message: "".to_string(),
        status: 0,
        data: None
    };

    let req = match req {
        Ok(data)=>{data},
        Err(err)=>{
            log::error!("validation  error  {}", err.to_string());
            resp_data.message = "Validation error".to_string();
            resp_data.status = 0;
            resp_data.data = None;
            return HttpResponse::InternalServerError().json( resp_data);
        }
    };
    return HttpResponse::Ok().json(resp_data)
}