use std::collections::HashMap;

use askeladd::config::Settings;
use askeladd::types::{FibonnacciProvingRequest, FibonnacciProvingResponse};
use askeladd::verifier_service::VerifierService;
use dotenv::dotenv;
use nostr_sdk::prelude::*;
use tokio::time::{sleep, Duration};
use uuid::Uuid;
pub use crate::types::marketplace::{ProverHandlerMarketplace, PaymentType};


pub async fn instantiate_client() -> Result<Client> {
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

    let opts = Options::new().wait_for_send(false);
    let client = Client::with_opts(&user_keys, opts);

    for relay in settings.subscribed_relays {
        client.add_relay(&relay).await?;
    }

    client.connect().await;

    Ok(client)
    
}



pub async fn add_client_subscription_notifications<F>(client:Client,  filter:Filter, sub_id:SubscriptionId,
    callback:F,

    // callback:Box<dyn Fn()>,
) -> Result<()> 

where 
F: Fn(Event) -> Result<bool>
{
    println!("User agent starting...");
    println!("Waiting 5 seconds before submitting proving request...");
    // Add a delay before connecting
    sleep(Duration::from_secs(5)).await;

    // Load configuration from .env file
    dotenv().ok();
    let settings = Settings::new().expect("Failed to load settings");

    let prover_agent_secret_key = SecretKey::from_bech32(&settings.prover_agent_sk)?;
    let prover_agent_keys = Keys::new(prover_agent_secret_key);
    println!("Client connected to relays.");

    // Generate a unique request ID

    client
        .subscribe_with_id(sub_id.clone(), vec![filter], None)
        .await;

    // Handle subscription notifications
    client
        .handle_notifications(|notification| async {
            if let RelayPoolNotification::Event {
                subscription_id,
                event,
                ..
            } = notification
            {
                if subscription_id == sub_id {
                    callback(*event);
                    // println!("Proving response received: {:?}", event);

                    // // Deserialize the response
                    // if let Ok(response) =
                    //     serde_json::from_str::<FibonnacciProvingResponse>(&event.content)
                    // {
                    //     // Verify the proof
                    //     let verifier_service: VerifierService = Default::default();
                    //     println!("Verifying proof...");
                    //     match verifier_service.verify_proof(response) {
                    //         Ok(_) => println!("Proof successfully verified"),
                    //         Err(e) => println!("Proof verification failed: {}", e),
                    //     }
                    //     // Stop listening after receiving and verifying the response
                    //     return Ok(true);
                    // }
                }
            }
            Ok(false)
        })
        .await?;

    Ok(())
}








async fn app_handler_custom_data(client:Client) -> Result<()> {
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

    client.connect().await;
    println!("Client connected to relays.");

    // Generate a unique request ID
    let request_id = Uuid::new_v4().to_string();

    // Create a proving request
    let proving_request = FibonnacciProvingRequest {
        request_id: request_id.clone(),
        log_size: 5,
        claim: 443693538,
    };



    // Add applications Event for a Program
    // NIP-78 - Arbitrary custom app data
    // https://nips.nostr.com/78
        // Serialize the request to JSON
    println!("Arbitrary custom app data");

    let request_json = serde_json::to_string(&proving_request)?;
    // let tags_request=vec![];
    let event_builder= EventBuilder::new(
        Kind::Custom(5000),
        request_json.clone(),
        vec![],
    );
    println!("event_builder {:?} ", event_builder);

    // let event_builder = EventBuilder::from(Kind::Custom(5000), tags_request).unwrap();
    let event= event_builder.to_event(&user_keys).unwrap();
    println!("event {:?} ", event);

    let event_job = client.send_event(event).await?;
    println!("event job {:?} ", event_job);

    
    // NIP-89 - Recommended Application Handlers
    // https://nips.nostr.com/89
    let main_payment=PaymentType::Zap;
    let mut min_paid_by_payment: HashMap<_, _, _>=HashMap::new();

    min_paid_by_payment.insert(PaymentType::Zap, 200_000);
    let min_paid=200_000;
    // let min_paid_by_payment=HashMap::from(PaymentType, 20000);
    println!("Recommended Application Handlers");
    let tags_app_handler:Vec<Tag>=vec![];

    let app_prover_handler= ProverHandlerMarketplace {
        request_id,
        main_payment,
        min_paid_by_payment,
        min_paid,
        url_storage:"".to_owned(),
        tags: tags_app_handler,
        input_metadata:proving_request.clone(),
        claim:min_paid,
        payment_accepted:vec![PaymentType::Zap],
        proof:Option::None

    };

    let request_app_json = serde_json::to_string(&app_prover_handler)?;

    // let tags_request=vec![];
    let event_builder_app_handler= EventBuilder::new(
        Kind::Custom(31989),
        request_app_json.clone(),
        vec![],
    );
    // let event_builder = EventBuilder::from(Kind::Custom(5000), tags_request).unwrap();
    let event= event_builder_app_handler.to_event(&user_keys).unwrap();
    println!("event App Prover Handler{:?} ", event);

    let event_app_handler = client.send_event(event).await?;
    println!("event_builder_app_handler {:?} ", event_app_handler);


    // TODO push PR Nostr SDK & Relayer to add this Kind::ApplicationHandler(u16)
    // let request_json = serde_json::to_string(&proving_request)?;
    // let tags_request=vec![];
    // let event_builder = EventBuilder::job_request(Kind::Custom(31989), tags_request).unwrap();
    // let event= event_builder.to_event(&user_keys).unwrap();
    // let event_job = client.send_event(event).await?;
    // println!("event job {:?} ", event_job);

    // TODO saved program in Storage :
    // Filecoin, Arweave or a Starknet DePin

    // NIP-90 - Data Vending Machine
    // https://nips.nostr.com/90
    // Job add prover marketplace
    println!("Publishing Prover Job handler...");

    let tags_request=vec![];
    let event_builder = EventBuilder::job_request(Kind::JobRequest(5000), tags_request).unwrap();
    let event= event_builder.to_event(&user_keys).unwrap();
    let event_job = client.send_event(event).await?;
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

    client
        .subscribe_with_id(proving_resp_sub_id.clone(), vec![filter], None)
        .await;

    // Handle subscription notifications
    client
        .handle_notifications(|notification| async {
            if let RelayPoolNotification::Event {
                subscription_id,
                event,
                ..
            } = notification
            {
                if subscription_id == proving_resp_sub_id {
                    println!("Proving response received: {:?}", event);

                    // Deserialize the response
                    if let Ok(response) =
                        serde_json::from_str::<FibonnacciProvingResponse>(&event.content)
                    {
                        // Verify the proof
                        let verifier_service: VerifierService = Default::default();
                        println!("Verifying proof...");
                        match verifier_service.verify_proof(response) {
                            Ok(_) => println!("Proof successfully verified"),
                            Err(e) => println!("Proof verification failed: {}", e),
                        }
                        // Stop listening after receiving and verifying the response
                        return Ok(true);
                    }
                }
            }
            Ok(false)
        })
        .await?;

    Ok(())
}




async fn event_recommended_application_handler(client:Client) -> Result<()> {
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

    client.connect().await;
    println!("Client connected to relays.");

    // Generate a unique request ID
    let request_id = Uuid::new_v4().to_string();

    // Create a proving request
    let proving_request = FibonnacciProvingRequest {
        request_id: request_id.clone(),
        log_size: 5,
        claim: 443693538,
    };



    // Add applications Event for a Program
    // NIP-78 - Arbitrary custom app data
    // https://nips.nostr.com/78
        // Serialize the request to JSON
    println!("Arbitrary custom app data");

    let request_json = serde_json::to_string(&proving_request)?;
    // let tags_request=vec![];
    let event_builder= EventBuilder::new(
        Kind::Custom(5000),
        request_json.clone(),
        vec![],
    );
    println!("event_builder {:?} ", event_builder);

    // let event_builder = EventBuilder::from(Kind::Custom(5000), tags_request).unwrap();
    let event= event_builder.to_event(&user_keys).unwrap();
    println!("event {:?} ", event);

    let event_job = client.send_event(event).await?;
    println!("event job {:?} ", event_job);

    
    // NIP-89 - Recommended Application Handlers
    // https://nips.nostr.com/89
    let main_payment=PaymentType::Zap;
    let mut min_paid_by_payment: HashMap<_, _, _>=HashMap::new();

    min_paid_by_payment.insert(PaymentType::Zap, 200_000);
    let min_paid=200_000;
    // let min_paid_by_payment=HashMap::from(PaymentType, 20000);
    println!("Recommended Application Handlers");
    let tags_app_handler:Vec<Tag>=vec![];

    let app_prover_handler= ProverHandlerMarketplace {
        request_id,
        main_payment,
        min_paid_by_payment,
        min_paid,
        url_storage:"".to_owned(),
        tags: tags_app_handler,
        input_metadata:proving_request.clone(),
        claim:min_paid,
        payment_accepted:vec![PaymentType::Zap],
        proof:Option::None

    };

    let request_app_json = serde_json::to_string(&app_prover_handler)?;

    // let tags_request=vec![];
    let event_builder_app_handler= EventBuilder::new(
        Kind::Custom(31989),
        request_app_json.clone(),
        vec![],
    );
    // let event_builder = EventBuilder::from(Kind::Custom(5000), tags_request).unwrap();
    let event= event_builder_app_handler.to_event(&user_keys).unwrap();
    println!("event App Prover Handler{:?} ", event);

    let event_app_handler = client.send_event(event).await?;
    println!("event_builder_app_handler {:?} ", event_app_handler);


    // TODO push PR Nostr SDK & Relayer to add this Kind::ApplicationHandler(u16)
    // let request_json = serde_json::to_string(&proving_request)?;
    // let tags_request=vec![];
    // let event_builder = EventBuilder::job_request(Kind::Custom(31989), tags_request).unwrap();
    // let event= event_builder.to_event(&user_keys).unwrap();
    // let event_job = client.send_event(event).await?;
    // println!("event job {:?} ", event_job);

    // TODO saved program in Storage :
    // Filecoin, Arweave or a Starknet DePin

    // NIP-90 - Data Vending Machine
    // https://nips.nostr.com/90
    // Job add prover marketplace
    println!("Publishing Prover Job handler...");

    let tags_request=vec![];
    let event_builder = EventBuilder::job_request(Kind::JobRequest(5000), tags_request).unwrap();
    let event= event_builder.to_event(&user_keys).unwrap();
    let event_job = client.send_event(event).await?;
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

    client
        .subscribe_with_id(proving_resp_sub_id.clone(), vec![filter], None)
        .await;

    // Handle subscription notifications
    client
        .handle_notifications(|notification| async {
            if let RelayPoolNotification::Event {
                subscription_id,
                event,
                ..
            } = notification
            {
                if subscription_id == proving_resp_sub_id {
                    println!("Proving response received: {:?}", event);

                    // Deserialize the response
                    if let Ok(response) =
                        serde_json::from_str::<FibonnacciProvingResponse>(&event.content)
                    {
                        // Verify the proof
                        let verifier_service: VerifierService = Default::default();
                        println!("Verifying proof...");
                        match verifier_service.verify_proof(response) {
                            Ok(_) => println!("Proof successfully verified"),
                            Err(e) => println!("Proof verification failed: {}", e),
                        }
                        // Stop listening after receiving and verifying the response
                        return Ok(true);
                    }
                }
            }
            Ok(false)
        })
        .await?;

    Ok(())
}





