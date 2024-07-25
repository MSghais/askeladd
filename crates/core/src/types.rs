use serde::{Deserialize, Serialize};
use stwo_prover::core::prover::StarkProof;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FibonnacciProvingRequest {
    pub request_id: String,
    pub log_size: u32,
    pub claim: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FibonnacciProvingResponse {
    pub request_id: String,
    pub log_size: u32,
    pub claim: u32,
    pub proof: StarkProof,
}

impl FibonnacciProvingResponse {
    pub fn new(request_id: String, log_size: u32, claim: u32, proof: StarkProof) -> Self {
        Self {
            request_id,
            log_size,
            claim,
            proof,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PoseidonProvingRequest {
    pub log_n_rows: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PoseidonProvingResponse {
    pub request_id: String,
    pub log_size: u32,
    pub claim: u32,
    pub proof: StarkProof,
}

impl PoseidonProvingResponse {
    pub fn new(request_id: String, log_size: u32, claim: u32, proof: StarkProof) -> Self {
        Self {
            request_id,
            log_size,
            claim,
            proof,
        }
    }
}

