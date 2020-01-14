use actix_web::{
    HttpResponse, Result
};
use crate::TEKSTER;


#[get("/syfotekster/internal/isAlive")]
pub async fn er_levende() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}

#[get("/syfotekster/internal/isReady")]
pub async fn er_klar() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}

#[get("/syfotekster/api/tekster")]
pub async fn tekster() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(&*TEKSTER))
}

#[get("/syfotekster/api/tekster/")]
pub async fn tekster_med_slash() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(&*TEKSTER))
}
