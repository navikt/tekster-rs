use actix_web::{
    HttpResponse, Result
};
use crate::TEKSTER;


#[get("/syfotekster/internal/isAlive")]
pub fn er_levende() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}

#[get("/syfotekster/internal/isReady")]
pub fn er_klar() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}

#[get("/syfotekster/api/tekster")]
pub fn tekster() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(&*TEKSTER))
}

#[get("/syfotekster/api/tekster/")]
pub fn tekster() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(&*TEKSTER))
}
