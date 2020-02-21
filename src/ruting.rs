use super::{Result, TEKSTER};
use hyper::{
    header, 
    Body, Method, Request, 
    Response, StatusCode
};

pub async fn ruter(req: Request<Body>) -> Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/syfotekster/api/tekster") | (&Method::GET, "/syfotekster/api/tekster/") => tekster().await,
        (&Method::GET, "/syfotekster/internal/isAlive") | (&Method::GET, "/syfotekster/internal/isReady") => okay(req).await,
        _ => ikke_funnet(req).await,
    }
}

async fn tekster() -> Result<Response<Body>> {
    info!("200 OK! Serverer tekster");
    Ok(
        Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(&TEKSTER[..]))
            .unwrap()
    )
}

async fn okay(req: Request<Body>) -> Result<Response<Body>> {
    info!("200 OK! {}", req.uri().path());
    Ok(
        Response::builder()
            .status(StatusCode::OK)
            .body("".into())
            .unwrap()
    )
}

async fn ikke_funnet(req: Request<Body>) -> Result<Response<Body>> {
    warn!("404 Not Found. {} er ikke ei gyldig rute", req.uri().path());
    Ok(
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("".into())
            .unwrap()
    )
}
