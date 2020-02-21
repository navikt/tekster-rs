#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
use hyper::{
    service::make_service_fn, service::service_fn, Server
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
type Result<T> = std::result::Result<T, GenericError>;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let addr: std::net::SocketAddr = ([0, 0, 0, 0], 8080).into();
    
    let ny_service = make_service_fn(move |_| {
         async {
             Ok::<_, GenericError>(service_fn(move |req| {
                 ruting::ruter(req)
             }))
         }
     });

     let server = Server::bind(&addr).serve(ny_service);
     info!("Lytter på http://{}", addr);
     server.await?;
     Ok(())
}

fn bygg_tekster() -> std::io::Result<HashMap<String, String>> {
    dotenv::dotenv().ok();

    let tekster_sti = std::env::var("TEKSTER_PATH").expect("TEKSTER_PATH");
    let tekster_sprak = std::env::var("TEKSTER_SPRAK").expect("TEKSTER_SPRAK");

    let mut tekst_map: HashMap<String, String> = HashMap::new();
    let fil_match = &format!("_{}", tekster_sprak);

    for sti in fs::read_dir(tekster_sti)? {
        if let Ok(tekstfil) = sti {
            if tekstfil.path().to_string_lossy().find(fil_match).is_some() {
                let mut fil = File::open(tekstfil.path())?;
                let mut innhold = String::new();
                fil.read_to_string(&mut innhold)?;

                let mut nokkel = match tekstfil.path().file_stem() {
                    Some(n) => match n.to_os_string().into_string() {
                        Ok(nokkel) => nokkel,
                        Err(_) => continue
                    },
                    None => continue
                };
                let lengde = nokkel.len();
                nokkel.truncate(lengde - 3);
                tekst_map.insert(nokkel, innhold);
            }
        }
    }
    Ok(tekst_map)
}
