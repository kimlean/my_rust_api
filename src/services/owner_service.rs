use mongodb::{error::Error, results::InsertOneResult, Collection};

use crate::models::owner_model::Owner;

pub struct OwnerService {
    owner: Collection<Owner>,
}

impl OwnerService {
    pub fn new(owner: Collection<Owner>) -> Self {
        Self { owner }
    }

    pub async fn create_owner(&self, owner: Owner) -> Result<InsertOneResult, Error> {
        let reslut = self
            .owner
            .insert_one(owner)
            .await
            .ok()
            .expect("Failed to insert owner");
    
        Ok(reslut)
    }
}