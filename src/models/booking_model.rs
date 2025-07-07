use std::time::SystemTime;

use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

use crate::models::owner_model::Owner;

use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(as = Booking)]
pub struct Booking {
    #[schema(value_type = String)]
    pub _id: ObjectId,
    #[schema(value_type = String)]
    pub owner: ObjectId,
    #[schema(value_type = String)]
    pub start_time: DateTime,
    pub duration_inminutes: u8,
    pub cancelled: bool
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BookingRequest {
    pub owner: String,
    pub start_time: String,
    pub duration_inminutes: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(as = FullBooking)]
pub struct FullBooking {
    #[schema(value_type = String)]
    pub _id: ObjectId,
    pub owner: Owner,
    pub dogs: Vec<String>,
    #[schema(value_type = String)]
    pub start_time: DateTime,
    pub duration_inminutes: u8,
    pub cancelled: bool
}

impl TryFrom<BookingRequest> for Booking {
    type Error = Box<dyn std::error::Error>;

    fn try_from(item: BookingRequest) -> Result<Self, Self::Error> {
        let chrono_datetime: SystemTime = chrono::DateTime::parse_from_rfc3339(&item.start_time)
            .map_err(|_| "Invalid start time format".to_string())?
            .with_timezone(&chrono::Utc)
            .into();

        Ok(Self{
            _id: ObjectId::new(),
            owner: ObjectId::parse_str(&item.owner).map_err(|_| "Invalid owner ID format".to_string())?,
            start_time: DateTime::from(chrono_datetime),
            duration_inminutes: item.duration_inminutes,
            cancelled: false,
        })

    }
    
}