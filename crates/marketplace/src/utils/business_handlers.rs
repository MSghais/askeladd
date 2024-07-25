use std::collections::HashMap;

use askeladd::config::Settings;
use askeladd::types::{FibonnacciProvingRequest, FibonnacciProvingResponse};
use dotenv::dotenv;
use nostr_sdk::prelude::*;
use tokio::time::{sleep, Duration};
use uuid::Uuid;

pub use crate::types::marketplace::{PaymentType, ProverHandlerMarketplace};

/// Add applications Event for a Program
/// NIP-78 - Arbitrary custom app data
/// https://nips.nostr.com/78
/// Serialize the request to JSON
async fn app_handler_custom_data<T>(
    client: Client,
    custom_app_stringify: String,
) -> Result<(EventId, Event)> {
    println!("app_handler_custom_data starting...");
    // Load configuration from .env file
    dotenv().ok();
    let settings = Settings::new().expect("Failed to load settings");
    let prover_agent_secret_key = SecretKey::from_bech32(&settings.prover_agent_sk)?;
    let user_keys = Keys::new(SecretKey::from_bech32(&settings.user_bech32_sk).unwrap());
    // Generate a unique request ID
    let request_id = Uuid::new_v4().to_string();

    println!("Arbitrary custom app data");
    let request_json = serde_json::to_string(&custom_app_stringify)?;
    // let tags_request=vec![];
    let event_builder = EventBuilder::new(Kind::Custom(5000), request_json.clone(), vec![]);
    let event = event_builder.to_event(&user_keys).unwrap();
    println!("event custom app{:?} ", event.clone());

    let event_custom_app = client.send_event(event.clone()).await?;
    println!("event custom_app {:?} ", event_custom_app);

    Ok((*event_custom_app.id(), event))
}

/// Data Vending Machines
/// https://nips.nostr.com/90
pub async fn data_vending_machine_result(
    client: Client,
    request_json: String,
    proving_request: String,
) -> Result<(EventId, Event)> {
    println!("User agent starting...");
    println!("Waiting 5 seconds before submitting proving request...");
    dotenv().ok();
    let settings = Settings::new().expect("Failed to load settings");
    let prover_agent_secret_key = SecretKey::from_bech32(&settings.prover_agent_sk)?;
    let prover_agent_keys = Keys::new(prover_agent_secret_key);
    let prover_agent_public_key = prover_agent_keys.public_key();
    let user_keys = Keys::new(SecretKey::from_bech32(&settings.user_bech32_sk).unwrap());
    println!("Client connected to relays.");
    let request_id = Uuid::new_v4().to_string();
    let main_payment = PaymentType::Zap;
    let mut min_paid_by_payment: HashMap<_, _, _> = HashMap::new();
    min_paid_by_payment.insert(PaymentType::Zap, 200_000);
    let min_paid = 200_000;
    println!("Recommended Application Handlers");
    let tags_app_handler: Vec<Tag> = vec![];
    let result_kind = 6000;

    let content = request_json.clone();
    let tags: Vec<Tag> = Vec::new();
    let event_result = EventBuilder::new(Kind::JobResult(result_kind), content, tags)
        .to_event(&prover_agent_keys)
        .unwrap();
    let event_builder = EventBuilder::job_result(event_result, min_paid, None).unwrap();
    let event = event_builder.to_event(&user_keys).unwrap();
    let event_job = client.send_event(event.clone()).await?;
    println!("event job {:?} ", event_job);

    // Publish the proving request
    println!("Publishing proving request...");
    let event_id = client.publish_text_note(request_json, []).await?;
    println!("Proving request published with event ID: {:?}", event_id);
    // Subscribe to proving responses
    let proving_resp_sub_id = SubscriptionId::new(settings.proving_resp_job_result_id);
    let filter = Filter::new()
        .kind(Kind::TextNote)
        .author(prover_agent_public_key)
        .since(Timestamp::now());

    Ok((*event_job.id(), event))
}

// Add applications Event for a Program
// NIP-78 - Arbitrary custom app data
// https://nips.nostr.com/78
// Serialize the request to JSON
async fn event_recommended_application_handler(
    client: Client,
    amount_millisats: u64,
) -> Result<EventId> {
    println!("Arbitrary custom app data");

    println!("User agent starting...");
    println!("Waiting 5 seconds before submitting proving request...");
    // Add a delay before connecting
    sleep(Duration::from_secs(5)).await;

    // Load configuration from .env file
    dotenv().ok();
    let settings = Settings::new().expect("Failed to load settings");
    let prover_agent_secret_key = SecretKey::from_bech32(&settings.prover_agent_sk)?;
    let prover_agent_keys = Keys::new(prover_agent_secret_key);
    let prover_agent_public_key = prover_agent_keys.public_key();
    let user_keys = Keys::new(SecretKey::from_bech32(&settings.user_bech32_sk).unwrap());

    // Generate a unique request ID
    let request_id = Uuid::new_v4().to_string();

    // Create a proving request
    let proving_request = FibonnacciProvingRequest {
        request_id: request_id.clone(),
        log_size: 5,
        claim: 443693538,
    };

    let request_json = serde_json::to_string(&proving_request)?;
    // let tags_request=vec![];
    let event_builder = EventBuilder::new(Kind::Custom(5000), request_json.clone(), vec![]);
    println!("event_builder {:?} ", event_builder);

    // let event_builder = EventBuilder::from(Kind::Custom(5000), tags_request).unwrap();
    let event = event_builder.to_event(&user_keys).unwrap();
    println!("event {:?} ", event);

    let event_job = client.send_event(event).await?;
    println!("event job {:?} ", event_job);

    // NIP-89 - Recommended Application Handlers
    // https://nips.nostr.com/89
    let main_payment = PaymentType::Zap;
    let mut min_paid_by_payment: HashMap<_, _, _> = HashMap::new();

    min_paid_by_payment.insert(PaymentType::Zap, amount_millisats);
    let min_paid = 200_000;
    // let min_paid_by_payment=HashMap::from(PaymentType, 20000);
    println!("Recommended Application Handlers");
    let tags_app_handler: Vec<Tag> = vec![];

    let app_prover_handler = ProverHandlerMarketplace {
        request_id,
        main_payment,
        min_paid_by_payment,
        min_paid,
        url_storage: "".to_owned(),
        tags: tags_app_handler,
        input_metadata: proving_request.clone(),
        claim: min_paid,
        payment_accepted: vec![PaymentType::Zap],
        proof: Option::None,
    };

    let request_app_json = serde_json::to_string(&app_prover_handler)?;
    let event_builder_app_handler =
        EventBuilder::new(Kind::Custom(31989), request_app_json.clone(), vec![]);
    let event = event_builder_app_handler.to_event(&user_keys).unwrap();
    println!("event App Prover Handler{:?} ", event);
    let event_app_handler = client.send_event(event).await?;
    println!("event_builder_app_handler {:?} ", event_app_handler);
    Ok(*event_app_handler.id())
}
