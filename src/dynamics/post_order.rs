// Construct and post a sales order to Microsoft Dynamics Business Central

use std::{collections::HashMap, io::Error};

use reqwest::Client;
use serde_json::{json, Value};

use super::structs::Order;

pub async fn post_order(token: String, order: Order) -> Result<Value, Error> {
    // API endpoint data 
    let tenant_id = ""; // Tenant ID > Obtained from Entra ID
    let company_id = ""; // Company ID from third party > Obtained from Microsoft Dynamics 
    let environment = ""; // Environment (Acceptance_V25 or Production)

    // API endpoint for creating a sales order
    let api_endpoint = format!(
        "https://api.businesscentral.dynamics.com/v2.0/{}/{}/api/MSA/API/v2.0/companies({})/salesOrders",
        tenant_id, environment, company_id
    );

    // Create JSON object for sales order
    // TODO: Add reference number to sales order
    let sales_order = json!({
        "customerNumber": "0007", // Customer ID (default value, same for every order) 
        "suitYourReference": order.study_id, // Study ID (Your reference inside Dynamics)
        "externalDocumentNumber": order.subject, // External document number
        "postingDate": order.due, // Due date (YYYY-MM-DD)
        "salesOrderLines": [ // Products/items in order
            {
                "lineType": "Item", // Line type (Item)
                "lineObjectNumber": order.head_path_size, // Head patch size (Article number)
                "quantity": 1, // Quantity (1 by default)
            }
        ],
        "shipToAddressLine1": order.ship_to_address_line, // Ship to address line 1
        "shipToCity": order.ship_to_city, // Ship to city
        "shipToCountry": order.ship_to_country, // Ship to country
        "shipToName": order.ship_to_name, // Ship to name
        "shipToPostCode": order.ship_to_postal_code, // Ship to postal code
        "phoneNumber": order.ship_to_phone, // Ship to phone
        "email": order.ship_to_email, // Ship to email
    
    });

    // Create a new reqwest client and post the sales order
    let client = Client::new(); 
    let response = match client
        .post(&api_endpoint)
        .bearer_auth(token)
        .json(&sales_order)
        .send()
        .await {
            Ok(response) => response,
            Err(e) => return Err(Error::new(std::io::ErrorKind::Other, e.to_string())),
        };
    
    // Check if the order is a success and return the order number
    if response.status().is_success() {
        let response_json: HashMap<String, Value> = match response.json().await {
            Ok(json) => json,
            Err(e) => return Err(Error::new(std::io::ErrorKind::Other, e.to_string())),
        };
        return response_json.get("number").cloned().ok_or(Error::new(std::io::ErrorKind::Other, "Failed to get order number"));
    } else {
        return Err(Error::new(std::io::ErrorKind::Other, format!("Failed to post order: {}", response.status())));
    }
}
