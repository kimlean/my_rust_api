use actix_web::{get, post, put, web::{Data, Json, Path}, HttpResponse};

use crate::{models::booking_model::{Booking, BookingRequest}, services::db::Database};

#[utoipa::path(
    post,
    path = "/booking",
    request_body = BookingRequest,
    responses(
        (status = 201, description = "Booking created successfully", body = Booking),
        (status = 500, description = "Failed to create booking")
    ),
    tag = "Bookings"
)]
#[post("/booking")]
pub async fn create_booking(db: Data<Database>, request: Json<BookingRequest>) -> HttpResponse {
    match db
        .create_booking(
            Booking::try_from(BookingRequest {
                owner: request.owner.clone(),
                start_time: request.start_time.clone(),
                duration_inminutes: request.duration_inminutes,
            })
            .expect("Failed to convert BookingRequest to Booking")
        )
        .await {
        Ok(result) => HttpResponse::Created().json(result),
        Err(e) => {
            eprintln!("Error creating booking: {}", e);
            HttpResponse::InternalServerError().body("Failed to create booking")
        }
    }
}

#[utoipa::path(
    get,
    path = "/bookings",
    responses(
        (status = 200, description = "List of all bookings", body = Vec<Booking>),
        (status = 500, description = "Failed to get bookings")
    ),
    tag = "Bookings"
)]
#[get("/bookings")]
pub async fn get_bookings(db: Data<Database>) -> HttpResponse{
    match db.get_bookings().await{
        Ok(booking) => HttpResponse::Ok().json(booking),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[utoipa::path(
    put,
    path = "/booking/{id}/cancel",
    responses(
        (status = 200, description = "Booking cancelled successfully"),
        (status = 500, description = "Failed to cancel booking")
    ),
    params(
        ("id" = String, Path, description = "Booking id")
    ),
    tag = "Bookings"
)]
#[put("/booking/{id}/cancel")]
pub async fn cancel_booking(db: Data<Database>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner(); // Extract the ID from the path

    match db.cancel_booking(&id).await {
        Ok(_) => HttpResponse::Ok().body("Booking cancelled successfully"),
        Err(e) => {
            eprintln!("Error cancelling booking: {}", e);
            HttpResponse::InternalServerError().body("Failed to cancel booking")
        }
    }
}
