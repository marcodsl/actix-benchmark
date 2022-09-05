use std::sync::Arc;

use actix_web::{
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder, Result,
};
use handler::{DynSieveHandler, SieveHandler};
use utils::sieve::{PrimeSolver, Sieve};

mod handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let prime_solver: Arc<dyn PrimeSolver + Send + Sync> = Arc::new(Sieve::new(1_000_000));
    let sieve_handler: Arc<DynSieveHandler> = Arc::new(SieveHandler::new(prime_solver.clone()));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::from(sieve_handler.clone()))
            .configure(|config| {
                let scope = web::scope("/primes").route("", web::get().to(get_primes));

                config.service(scope);
            })
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

pub async fn get_primes(handler: Data<DynSieveHandler>) -> Result<impl Responder> {
    let response = handler.execute(()).await?;

    Ok(HttpResponse::Ok().body(format!("{:?}", response)))
}
