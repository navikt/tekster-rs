use super::*;

pub fn ruter(req: Request<Body>, _client: &Client<HttpConnector>) -> ResponseFuture {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/syfotekster/api/tekster") | (&Method::GET, "/syfotekster/api/tekster/") => tekster(),
        (&Method::GET, "/syfotekster/internal/isAlive") | (&Method::GET, "/syfotekster/internal/isReady") => okay(),
        _ => ikke_funnet(),
    }
}

fn tekster() -> ResponseFuture {
    let tekster = Body::from(&TEKSTER[..]);
    Box::new(future::ok(
        Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .body(tekster)
            .unwrap(),
    ))
}

fn okay() -> ResponseFuture {
    let body = Body::from("");
    Box::new(future::ok(
        Response::builder()
            .status(StatusCode::OK)
            .body(body)
            .unwrap(),
    ))
}

fn ikke_funnet() -> ResponseFuture {
    let body = Body::from("");
    Box::new(future::ok(
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(body)
            .unwrap(),
    ))
}
