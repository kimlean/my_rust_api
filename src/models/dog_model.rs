use mongodb::bson::{oid::ObjectId};
use serde::{Deserialize, Serialize};

use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(as = Dog)]
pub struct Dog {
    #[schema(value_type = String)]
    pub _id: ObjectId,
    #[schema(value_type = String)]
    pub owner: ObjectId,
    pub name: Option<String>,
    pub age: Option<u8>,
    pub breed: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DogRequest {
    pub owner: String,
    pub name: Option<String>,
    pub age: Option<u8>,
    pub breed: Option<String>
}

impl TryFrom<DogRequest> for Dog {
    type Error = Box<dyn std::error::Error>;

    fn try_from(item: DogRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            _id: ObjectId::new(),
            owner: ObjectId::parse_str(&item.owner).map_err(|_| "Invalid owner ID format".to_string())?,
            name: item.name,
            age: item.age,
            breed: item.breed,
        })
    }

}