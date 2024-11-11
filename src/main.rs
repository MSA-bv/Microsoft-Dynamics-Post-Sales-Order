use dynamics::{post_order::post_order, structs::Order};

mod auth;
mod dynamics;

#[tokio::main]
async fn main() {
    // Get the AUTH token
    let token = match auth::token::request_token().await {
        Ok(token) => token,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    // Create an order
    let order = Order {
        subject: "12345".to_string(), // External document number supplied by other party (12345 for testing)
        _study_id: "XXXXXX".to_string(), // TODO: Supply  reference. Actual study ID supplied by other party (9x9 for testing) 
        due: "2025-01-01".to_string(), // Due date supplied by other party, format "YYYY-MM-DD" (2025-01-01 for testing) 
        head_path_size: "00000".to_string(), // Head Path Size supplied by other party (51491 for testing)
        ship_to_name: "John Doe".to_string(), // Ship To Name supplied by other party (Lodewijk Boon for testing)
        ship_to_address_line: "PC hoofdstraat".to_string(), // Ship To Address supplied by other party (Euroweg 21 for testing)
        ship_to_city: "Amsterdam".to_string(), // Ship To City supplied by other party (Amersfoort for testing)
        ship_to_postal_code: "1111AA".to_string(), // Ship To Postal Code supplied by other party (3825HA for testing)
        ship_to_country: "NL".to_string(), // Ship To Country supplied by other party (NL for testing)
        ship_to_phone: "0612345678".to_string(), // Ship To Phone supplied by other party (0612345678 for testing)
        ship_to_email: "johndoe@gmail.com".to_string(), // Ship To Email supplied by other party (johndoe@gmail.com for testing)
    };

    // Post the order
    match post_order(token, order).await {
        Ok(order_number) => println!("Order number: {}", order_number),
        Err(e) =>  { 
            println!("Error: {}", e);
            return;
        }
    }
}
