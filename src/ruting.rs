use actix_web::{
    HttpResponse, Result
};
use crate::TEKSTER;


#[get("/is_alive")]
pub fn er_levende() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}

#[get("/is_ready")]
pub fn er_klar() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}

#[get("/tekster")]
pub fn tekster() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(&*TEKSTER))
}
