use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(pub String);

impl From<&str> for UserId {
    fn from(s: &str) -> Self {
        UserId(s.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ServiceId(pub String);
impl From<&str> for ServiceId {
    fn from(s: &str) -> Self {
        ServiceId(s.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProductId(pub String);
impl From<&str> for ProductId {
    fn from(s: &str) -> Self {
        ProductId(s.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub profile: Profile,
}

impl User {
    pub fn new(id: &str, display_name: &str, default_payment: Option<PaymentMethod>) -> Self {
        User {
            id: UserId(id.to_string()),
            profile: Profile {
                display_name: display_name.to_string(),
                default_payment,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub display_name: String,
    pub default_payment: Option<PaymentMethod>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentMethod {
    Card {
        card_number_masked: String,
        holder: String,
    },
    Paypal {
        account: String,
    },
}

impl PaymentMethod {
    pub fn card(mask: &str, holder: &str) -> Self {
        PaymentMethod::Card {
            card_number_masked: mask.to_string(),
            holder: holder.to_string(),
        }
    }
    pub fn paypal(account: &str) -> Self {
        PaymentMethod::Paypal {
            account: account.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: ProductId,
    pub name: String,
    pub price_cents: u64,
}

impl Product {
    pub fn new(id: &str, name: &str, price_cents: u64) -> Self {
        Product {
            id: ProductId(id.to_string()),
            name: name.to_string(),
            price_cents,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub id: ServiceId,
    pub name: String,
    pub products: Vec<Product>,
}

impl Service {
    pub fn new(id: &str, name: &str, products: Vec<Product>) -> Self {
        Service {
            id: ServiceId(id.to_string()),
            name: name.to_string(),
            products,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceUsage {
    pub user_id: UserId,
    pub service_id: ServiceId,
    pub product_id: ProductId,
    pub payment_used: Option<PaymentMethod>,
}

impl ServiceUsage {
    pub fn new(
        user_id: &UserId,
        service_id: &ServiceId,
        product_id: &ProductId,
        payment_used: Option<PaymentMethod>,
    ) -> Self {
        ServiceUsage {
            user_id: user_id.clone(),
            service_id: service_id.clone(),
            product_id: product_id.clone(),
            payment_used,
        }
    }
}

// small helper types for collections
pub type ServiceMap = HashMap<ServiceId, Service>;
