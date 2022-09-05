use std::error::Error;

use utils::{handler::Handler, sieve::PrimeSolver};

pub type DynSieveHandler = dyn Handler<(), Response = Vec<usize>>;

pub struct SieveHandler<S> {
    pub prime_solver: S,
}

impl<S> SieveHandler<S>
where
    S: PrimeSolver + Send + Sync,
{
    pub fn new(prime_solver: S) -> Self {
        Self { prime_solver }
    }
}

#[async_trait::async_trait]
impl<S> Handler<()> for SieveHandler<S>
where
    S: PrimeSolver + Send + Sync,
{
    type Response = Vec<usize>;

    async fn execute(&self, _: ()) -> Result<Self::Response, Box<dyn Error>> {
        Ok(self.prime_solver.primes())
    }
}
