use stwo_prover::core::fields::m31::BaseField;
use stwo_prover::core::prover::ProvingError;
use stwo_prover::examples::fibonacci::Fibonacci;
use askeladd_core::types::{FibonnacciProvingRequest, FibonnacciProvingResponse};
use stwo_prover::examples::poseidon::PoseidonAir;

#[derive(Debug, Default)]
pub struct ProverService {}

impl ProverService {
    pub fn generate_proof(
        &self,
        request: FibonnacciProvingRequest,
    ) -> Result<FibonnacciProvingResponse, ProvingError> {
        let fib = Fibonacci::new(request.log_size, BaseField::from(request.claim));
        match fib.prove() {
            Ok(proof) => Ok(FibonnacciProvingResponse::new(
                request.request_id,
                request.log_size,
                request.claim,
                proof,
            )),
            Err(e) => Err(e),
        }
    }
}
