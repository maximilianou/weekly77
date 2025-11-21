use crate::models::{Product, Service, ServiceId, ServiceMap};

#[derive(Debug, Clone, Default)]
pub struct Catalog {
    pub services: ServiceMap,
}

impl Catalog {
    pub fn with_service(mut self, service: Service) -> Self {
        self.services.insert(service.id.clone(), service);
        self
    }

    pub fn list_products_for_service(&self, service_id: &ServiceId) -> Option<Vec<Product>> {
        self.services.get(service_id).map(|s| s.products.clone())
    }

    pub fn list_all_products(&self) -> Vec<Product> {
        self.services
            .values()
            .flat_map(|s| s.products.clone())
            .collect()
    }

    #[allow(dead_code)]
    pub fn get_service(&self, service_id: &ServiceId) -> Option<Service> {
        self.services.get(service_id).cloned()
    }
}
