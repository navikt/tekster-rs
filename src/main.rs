#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
use hyper::{
    service::make_service_fn, service::service_fn, Server
};

mod ruting;
mod tekster;

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

