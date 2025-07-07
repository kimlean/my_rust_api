use actix_web::{post, web::{Data, Json}, HttpResponse};

use crate::{models::dog_model::{Dog, DogRequest}, services::db::Database};

#[utoipa::path(
    post,
    path = "/dog",
    request_body = DogRequest,
    responses(
        (status = 201, description = "Dog created successfully", body = Dog),
        (status = 500, description = "Failed to create dog")
    ),
    tag = "Dogs"
)]
#[post("/dog")]
pub async fn create_dog(db: Data<Database>, request: Json<DogRequest>) -> HttpResponse {
    match db
        .create_dog(
            Dog::try_from(DogRequest {
                owner: request.owner.clone(),
                name: request.name.clone(),
                age: request.age,
                breed: request.breed.clone(),
            })
            .expect("Failed to convert DogRequest to Dog")
        )
        .await {
        Ok(result) => HttpResponse::Created().json(result),
        Err(e) => {
            eprintln!("Error creating dog: {}", e);
            HttpResponse::InternalServerError().body("Failed to create dog")
        }
    }

}