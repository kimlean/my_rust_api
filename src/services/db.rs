use std::{str::FromStr, time::SystemTime};
use futures_util::StreamExt;
use mongodb::{bson::{doc, from_document, oid::ObjectId, DateTime}, error::Error, results::{InsertOneResult, UpdateResult}, Collection};
use crate::models::{booking_model::{Booking, FullBooking}, dog_model::Dog, owner_model::Owner};

pub struct Database {
    dog: Collection<Dog>,
    owner: Collection<Owner>,
    booking: Collection<Booking>,
}

impl Database {
    pub async fn init() -> Self {
        let client = mongodb::Client::with_uri_str("mongodb://localhost:27017").await.unwrap();
        let db = client.database("dog_walking");

        let dog = db.collection::<Dog>("dog");
        let owner = db.collection::<Owner>("owner");
        let booking = db.collection::<Booking>("booking");

        Self { dog, owner, booking }
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

    pub async fn create_dog(&self, dog: Dog) -> Result<InsertOneResult, Error> {
        let result = self
            .dog
            .insert_one(dog)
            .await
            .ok()
            .expect("Failed to insert dog");
    
        Ok(result)
    }

    pub async fn create_booking(&self, booking: Booking) -> Result<InsertOneResult, Error> {
        let result = self
            .booking
            .insert_one(booking)
            .await
            .ok()
            .expect("Failed to insert booking");
    
        Ok(result)
    }

    pub async fn cancel_booking(&self, booking_id: &str) -> Result<UpdateResult, Error> {
        let result = self
            .booking
            .update_one(
                doc! { "_id": ObjectId::from_str(booking_id).unwrap() },
                doc! { "$set": { "cancelled": true } }
            )
            .await
            .ok()
            .expect("Failed to cancel booking");

        Ok(result)
    }

    pub async fn get_bookings(&self) -> Result<Vec<FullBooking>, Error> {
        let now: SystemTime = SystemTime::now();
        let mut results = self
            .booking
            .aggregate(vec![
                doc! { "$match" : { "cancelled" : false, "start_time": { "$gte": DateTime::from_system_time(now) } } },
                doc! { "$lookup": { "from": "owner", "localField": "owner", "foreignField": "_id", "as": "owner" }},
                doc! { "$unwind": doc! {"path" : "$owner"} },
                doc! { "$lookup": { "from": "dog", "localField": "owner._id", "foreignField": "owner", "as": "dogs" }},
            ])
            .await
            .ok()
            .expect("Failed to get bookings");

        let mut bookings: Vec<FullBooking> = Vec::new();
        while let Some(result) = results.next().await {
            match result {
                Ok(doc) => {
                    let booking: FullBooking = from_document(doc).expect("Failed to convert document to FullBooking");
                    bookings.push(booking);
                },
                Err(e) => {
                    eprintln!("Error processing booking: {}", e);
                }
            }
        }

        Ok(bookings)
    }

}
