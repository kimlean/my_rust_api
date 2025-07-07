use mongodb::{error::Error, results::InsertOneResult, Collection};

use crate::models::dog_model::Dog;

pub struct DogService {
    dog: Collection<Dog>,
}

impl DogService {
    pub fn new(dog: Collection<Dog>) -> Self {
        Self { dog }
    }

    pub async fn create_dog(&self, dog: Dog) -> Result<InsertOneResult, Error> {
        let result = self
            .dog
            .insert_one(dog)
            .await
            .ok()
            .expect("Failed to insert dog");
    
        Ok(result)
    }
}