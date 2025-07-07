use crate::{
    models::{booking_model::Booking, dog_model::Dog, owner_model::Owner},
    services::{
        booking_service::BookingService, dog_service::DogService, owner_service::OwnerService,
    },
};
use mongodb::Client;

pub struct Database {
    pub owner: OwnerService,
    pub dog: DogService,
    pub booking: BookingService,
}

impl Database {
    pub async fn init() -> Self {
        let client = Client::with_uri_str("mongodb://172.17.199.39:27017")
            .await
            .unwrap();
        let db = client.database("dog_walking");

        let owner_collection = db.collection::<Owner>("owner");
        let dog_collection = db.collection::<Dog>("dog");
        let booking_collection = db.collection::<Booking>("booking");

        let owner_service = OwnerService::new(owner_collection);
        let dog_service = DogService::new(dog_collection);
        let booking_service = BookingService::new(booking_collection);

        Self {
            owner: owner_service,
            dog: dog_service,
            booking: booking_service,
        }
    }
}