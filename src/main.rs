use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod models;
mod services;
mod routes;

use actix_web::{web::Data, App, HttpServer};

use crate::routes::{
    booking_route::{cancel_booking, create_booking, get_bookings}, 
    dog_route::create_dog, 
    owner_route::create_owner,
    health_route::health_check
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    #[derive(OpenApi)]
    #[openapi(
        paths(
            routes::booking_route::create_booking,
            routes::booking_route::get_bookings,
            routes::booking_route::cancel_booking,
            routes::dog_route::create_dog,
            routes::owner_route::create_owner
        ),
        components(
            schemas(
                models::booking_model::Booking,
                models::booking_model::BookingRequest,
                models::booking_model::FullBooking,
                models::dog_model::Dog,
                models::dog_model::DogRequest,
                models::owner_model::Owner,
                models::owner_model::OwnerRequest
            )
        ),
        tags(
            (name = "Dog Booking API", description = "A simple API to book a dog")
        )
    )]
struct ApiDoc;

    // Initialize the database connection
    let db = services::db::Database::init().await;
    let db_data = Data::new(db);

    HttpServer::new(
        move || App::new()
        .app_data(db_data.clone())
        .service(create_booking)
        .service(create_dog)
        .service(create_owner)
        .service(get_bookings)
        .service(cancel_booking)
        .service(health_check)
        .service(
            SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
    )
    .bind("localhost:8080")?
    .run()
    .await
}