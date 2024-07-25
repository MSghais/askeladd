use askeladd_core::config::Settings;
use dotenv::dotenv;
use nostr_sdk::prelude::*;
use tokio::time::{sleep, Duration};
pub use crate::types::marketplace::{ProverHandlerMarketplace, PaymentType};

// NIP-90 - Data Vending Machine
// https://nips.nostr.com/90
// Job add prover marketplace
pub async fn data_vending_machine_request(client:Client, request_json:String) -> Result<(EventId,Event)> {
    println!("User starting data_vending_machine_request...");
    // Add a delay before connecting
    sleep(Duration::from_secs(5)).await;

    // Load configuration from .env file
    dotenv().ok();
    let settings = Settings::new().expect("Failed to load settings");

    let prover_agent_secret_key = SecretKey::from_bech32(&settings.prover_agent_sk)?;
    let prover_agent_keys = Keys::new(prover_agent_secret_key);
    let prover_agent_public_key = prover_agent_keys.public_key();
    let user_keys = Keys::new(SecretKey::from_bech32(&settings.user_bech32_sk).unwrap());

 
    println!("Publishing Prover Job handler...");

    let tags_request=vec![];
    let event_builder = EventBuilder::job_request(Kind::JobRequest(5000), tags_request).unwrap();
    let event= event_builder.to_event(&user_keys).unwrap();
    let event_job = client.send_event(event.clone()).await?;
    println!("event job {:?} ", event_job);

    Ok((*event_job.id(), event))
}
