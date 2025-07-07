use std::{str::FromStr, time::SystemTime};

use chrono::Utc;
use futures_util::StreamExt;
use mongodb::{bson::{doc, from_document, oid::ObjectId, DateTime}, error::Error, results::{InsertOneResult, UpdateResult}, Collection};

use crate::models::booking_model::{Booking, FullBooking};

pub struct BookingService {
    booking: Collection<Booking>,
}

impl BookingService {
    pub fn new(booking: Collection<Booking>) -> Self {
        Self { booking }
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
        let now: SystemTime = Utc::now().into();
        let mut results = self
            .booking
            .aggregate(vec![
                doc! { "$match" : { "cancelled" : false, "start_time": { "$gte": DateTime::from_system_time(now) }}},
                doc! { "$lookup": doc! { "from": "owner", "localField": "owner", "foreignField": "_id", "as": "owner" }},
                doc! { "$unwind": doc! {"path" : "$owner"} },
                doc! { "$lookup": doc! { "from": "dog", "localField": "owner._id", "foreignField": "owner", "as": "dogs"}},
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