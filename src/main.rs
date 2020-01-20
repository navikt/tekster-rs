#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
use futures::{future, Future};
use hyper::{
    client::HttpConnector, rt, service::service_fn, Body, Client, Request,
    Response, Server, Method, StatusCode, header
};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

mod ruting;

lazy_static! {
    static ref TEKSTER: String = serde_json::to_string(&bygg_tekster().unwrap()).unwrap();
}

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type ResponseFuture = Box<dyn Future<Item = Response<Body>, Error = GenericError> + Send>;

fn main() {
    env_logger::init();
    let addr: std::net::SocketAddr = "0.0.0.0:8080".parse().unwrap();

    rt::run(future::lazy(move || {
        let klient = Client::new();

        let ny_service = move || {
            let klient = klient.clone();
            service_fn(move |request| ruting::ruter(request, &klient))
        };

        let server = Server::bind(&addr)
            .serve(ny_service)
            .map_err(|e| eprintln!("Server error: {}", e));
            info!("Lytter på {}", addr);
            server
        }));
}

fn bygg_tekster() -> std::io::Result<HashMap<String, String>> {
    dotenv::dotenv().ok();

    let tekster_sti = std::env::var("TEKSTER_PATH").expect("TEKSTER_PATH");
    let tekster_sprak = std::env::var("TEKSTER_SPRAK").expect("TEKSTER_SPRAK");

    let mut tekst_map: HashMap<String, String> = HashMap::new();
    let fil_match = &format!("_{}", tekster_sprak);

    for s in fs::read_dir(tekster_sti)? {
        if let Ok(sti) = s {
            if sti.path().to_string_lossy().find(fil_match).is_some() {
                let mut nokkel = sti.path().file_stem().unwrap().to_os_string().into_string().unwrap();
                let lengde = nokkel.len();
                nokkel.truncate(lengde - 3);
                let mut fil = File::open(sti.path())?;
                let mut innhold = String::new();
                fil.read_to_string(&mut innhold)?;
                tekst_map.insert(nokkel, innhold);
            }
        }
    }
    Ok(tekst_map)
}
