use crate::models::{PaymentMethod, ServiceUsage, User};

#[derive(Debug, Clone, Default)]
pub struct UsageLog {
    pub usages: Vec<ServiceUsage>,
}

impl UsageLog {
    pub fn from_vec(usages: Vec<ServiceUsage>) -> Self {
        UsageLog { usages }
    }

    pub fn service_usages_for_user(&self, user_id: &crate::models::UserId) -> Vec<ServiceUsage> {
        self.usages
            .iter()
            .filter(|u| &u.user_id == user_id)
            .cloned()
            .collect()
    }

    pub fn add_usage(&self, usage: ServiceUsage) -> UsageLog {
        let mut new = self.clone();
        new.usages.push(usage);
        new
    }

    pub fn usages_len(&self) -> usize {
        self.usages.len()
    }
}

// Clone is derived above; no manual impl required.

// pure helper: resolve payment for a usage using user's default if missing
pub fn resolve_payment_for_usage(
    user: &User,
    usage_payment: Option<PaymentMethod>,
) -> Option<PaymentMethod> {
    usage_payment.or_else(|| user.profile.default_payment.clone())
}
