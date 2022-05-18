extern crate dotenv;

use tide::Request;
use tide::{Body};
use dotenv::dotenv;
use std::env;
use vrf::openssl::{CipherSuite, ECVRF};
use vrf::VRF;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct VRFRequestBody {
    message_hex: String,
}

#[derive(Debug, Serialize)]
struct VRFResponse {
    message_hex: String,
    pi_hex: String,
    hash_hex: String,
    pub_hex: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv().ok();
    let mut app = tide::new();

    // Configure routes
    app.at("/").get(handle_root_request);
    app.at("/vrf-requests").post(handle_vrf_request);

    // Run web server at port 8080
    println!("Running VRF web server on port: 8080");
    app.listen("127.0.0.1:8080").await?;

    Ok(())
}

async fn handle_vrf_request(mut req: Request<()>) -> tide::Result {
    // Extract request details
    let VRFRequestBody { message_hex } = req.body_json().await?;
    println!("Received VRF request with message: {}", message_hex);

    // Convert hex to bytes
    let message_bytes = hex::decode(message_hex).unwrap();
    let private_key_bytes = get_private_key_bytes().unwrap();

    // Calculate VRF
    let mut vrf = get_vrf_instance();
    let pi_bytes = vrf.prove(&private_key_bytes, &message_bytes).unwrap();
    let hash_bytes = vrf.proof_to_hash(&pi_bytes).unwrap();
    let pub_hex = get_node_public_key_hex();

    // Prepare VRF response details
    let vrf_response = VRFResponse {
        message_hex: to_hex_string(message_bytes),
        pi_hex: to_hex_string(pi_bytes),
        hash_hex: to_hex_string(hash_bytes),
        pub_hex,
    };
    println!("Prepared VRF response: {:?}", vrf_response);

    // Prepare HTTP JSON response
    let mut http_response = tide::Response::new(200);
    http_response.set_body(Body::from_json(&vrf_response).unwrap());
    Ok(http_response)
}

async fn handle_root_request(_req: Request<()>) -> tide::Result {
    println!("Received root request");
    let public_key_hex = get_node_public_key_hex();
    let response_msg = format!("I am RedStone VRF Node. My public key: {}", public_key_hex);
    Ok(response_msg.into())
}

fn get_node_public_key_hex() -> String {
    let mut vrf = get_vrf_instance();
    let private_key = get_private_key_bytes().unwrap();
    let public_key = vrf.derive_public_key(&private_key).unwrap();
    to_hex_string(public_key)
}

fn get_vrf_instance() -> ECVRF {
    ECVRF::from_suite(CipherSuite::SECP256K1_SHA256_TAI).unwrap()
}

fn get_private_key_bytes() -> Result<Vec<u8>, hex::FromHexError> {
    let private_key_hex = env::var("PRIVATE_KEY").unwrap();
    let private_key_bytes = hex::decode(private_key_hex).unwrap();
    Ok(private_key_bytes)
}

fn to_hex_string(bytes: Vec<u8>) -> String {
    bytes
        .iter()
        .fold(String::from("0x"), |acc, x| format!("{}{:02x}", acc, x))
}
