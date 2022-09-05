use std::{error::Error, sync::Arc};

use utils::{handler::Handler, sieve::PrimeSolver};

pub type DynSieveHandler = dyn Handler<(), Response = Vec<usize>>;

pub struct SieveHandler {
    pub prime_solver: Arc<dyn PrimeSolver + Send + Sync>,
}

impl SieveHandler {
    pub fn new(prime_solver: Arc<dyn PrimeSolver + Send + Sync>) -> Self {
        Self { prime_solver }
    }
}

#[async_trait::async_trait]
impl Handler<()> for SieveHandler {
    type Response = Vec<usize>;

    async fn execute(&self, _: ()) -> Result<Self::Response, Box<dyn Error>> {
        Ok(self.prime_solver.primes())
    }
}
