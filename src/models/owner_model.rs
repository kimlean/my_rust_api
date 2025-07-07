use mongodb::bson::{oid::ObjectId};
use serde::{Deserialize, Serialize};

use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(as = Owner)]
pub struct Owner {
    #[schema(value_type = String)]
    pub _id: ObjectId,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OwnerRequest {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String
}

impl TryFrom<OwnerRequest> for Owner {
    type Error = Box<dyn std::error::Error>;

    fn try_from(item: OwnerRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            _id: ObjectId::new(),
            name: item.name,
            email: item.email,
            phone: item.phone,
            address: item.address,
        })
    }
}