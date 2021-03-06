use crate::types::primitive::Integer;
use serde::Deserialize;

/// Phone contact
#[derive(Clone, Debug, Deserialize)]
pub struct Contact {
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Contact's last name
    pub last_name: Option<String>,
    /// Contact's user identifier in Telegram
    pub user_id: Option<Integer>,
    /// Additional data about the contact in the form of a vCard
    pub vcard: Option<String>,
}
