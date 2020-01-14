#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate lazy_static;

use actix_web::{
    middleware, App, HttpServer
};
use dotenv;
use std::collections::HashMap;
use std::fs::File;
use std::fs;
use std::io::prelude::*;

mod ruting;

lazy_static! {
    static ref TEKSTER: String = serde_json::to_string(&bygg_tekster().unwrap()).unwrap();
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .service(ruting::er_klar)
            .service(ruting::er_levende)
            .service(ruting::tekster)
            .service(ruting::tekster_med_slash)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await

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
