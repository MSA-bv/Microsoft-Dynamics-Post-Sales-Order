use std::{collections::HashMap, io::{Error, ErrorKind}};

use reqwest::{Client, Response};
use serde::Deserialize;
use serde_json::from_str;

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
}

pub async fn request_token() -> Result<String, Error> {
    // Dynamics API data, retreived from Entra > App Registrations > Onera API
    let tenant_id = ""; // Tenant ID > Obtained from Entra > App Registrations > Onera API access
    let service_principal_id = ""; // Service Principal // Obtained from Entra > App Registrations > Onera API access
    let client_secret = ""; // Onera API secret
    
    // Token endpoint & scope
    let token_endpoint = format!("https://login.microsoftonline.com/{}/oauth2/v2.0/token", tenant_id);
    let scope = "https://api.businesscentral.dynamics.com/.default";

    // Create a HashMap to store the request body
    let mut params = HashMap::new();
    params.insert("client_id", service_principal_id);
    params.insert("scope", scope);
    params.insert("client_secret", client_secret);
    params.insert("grant_type", "client_credentials");

    // Create a new reqwest client and post the request
    let client = Client::new();
    let response = match client
        .post(&token_endpoint)
        .form(&params)
        .send()
        .await {
            Ok(response) => response,
            Err(e) => return Err(Error::new(ErrorKind::Other, e.to_string())),
        };

    // Decode the response
    let response_json = match decode_response(response).await {
        Ok(json) => json,
        Err(e) => return Err(e),
    };

    let token = response_json.access_token;

    // Return token as string
    Ok(token.to_string())
} 

// Decode the response
async fn decode_response(response: Response) -> Result<TokenResponse, Error> {
    if !response.status().is_success() {
        return Err(Error::new(ErrorKind::Other, format!("HTTP error: {}", response.status())));
    }

    let raw_body = match response.text().await {
        Ok(body) => body,
        Err(e) => return Err(Error::new(ErrorKind::InvalidData, format!("Error while reading response body: {}", e.to_string())))
    };

    let response_json: TokenResponse = match from_str(&raw_body) {
        Ok(json) => json,
        Err(e) => return Err(Error::new(ErrorKind::InvalidData, format!("Error while decoding text body: {}", e.to_string())))
    };

    Ok(response_json)
}