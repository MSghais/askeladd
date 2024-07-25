use std::collections::HashMap;
use std::hash::Hash;
use nostr_sdk::{EventId, Tag};
use serde::{Deserialize, Serialize, Serializer};
use stwo_prover::core::prover::StarkProof;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub enum PaymentType {
    Zap,
    Starknet,
}
pub trait SomeTrait {}


#[derive(Debug, Serialize, Deserialize)]
pub struct ProverHandlerMarketplace<T> {
    pub request_id: String,
    // pub event_id: Option<EventId>,
    pub claim: u32,
    pub proof: Option<StarkProof>,
    pub input_metadata: T,
    pub tags:Vec<Tag>,
    pub url_storage: String,
    // Payment
    pub min_paid: u32,
    pub min_paid_by_payment: HashMap<PaymentType, u64>,
    pub payment_accepted: Vec<PaymentType>,
 
    pub main_payment:PaymentType,
}
/// Constraint for the type parameter `T` in MyStruct

impl <T>ProverHandlerMarketplace <T>{
    pub fn new(
        request_id: String,
        // event_id: EventId,
        min_paid: u32,
        claim: u32,
        proof: Option<StarkProof>,
        min_paid_by_payment: HashMap<PaymentType, u64>,
        url_storage: String,
        payment_accepted: Vec<PaymentType>,
        input_metadata: T,
        tags:Vec<Tag>,
        main_payment:PaymentType
    ) -> Self {
        Self {
            request_id,
            // event_id,
            min_paid,
            claim,
            proof,
            min_paid_by_payment,
            url_storage,
            payment_accepted,
            input_metadata,
            tags,
            main_payment
        }
    }
}
