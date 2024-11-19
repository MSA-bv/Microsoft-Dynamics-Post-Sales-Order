// Struct with all the variable order date to be pushed to the API

use serde::Deserialize;

// Struct to push order data to, used internal termonology (instead of Dynamics terminology)
#[derive(Deserialize)]
pub struct Order {
    pub study_id: String, // Uw referentie / your reference (UNUSED BECAUSE FIELD MISSES IN API!!!)
    pub subject: String, // externalDocumentNumber
    pub due: String, // postingDate
    pub head_path_size: String, // lineObjectNumber
    pub ship_to_name: String, // shipToName
    pub ship_to_address_line: String, // shipToAddressLine1
    pub ship_to_city: String, // shipToCity
    pub ship_to_postal_code: String, // shipToPostCode
    pub ship_to_country: String, // shipToCountry
    pub ship_to_phone: String, // phoneNumber
    pub ship_to_email: String, // email
}