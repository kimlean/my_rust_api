use actix_web::{post, web::{Data, Json}, HttpResponse};

use crate::{models::owner_model::{Owner, OwnerRequest}, util::db::Database};

#[utoipa::path(
    post,
    path = "/owner",
    request_body = OwnerRequest,
    responses(
        (status = 201, description = "Owner created successfully", body = Owner),
        (status = 500, description = "Failed to create owner")
    ),
    tag = "Owners"
)]
#[post("/owner")]
pub async fn create_owner(db: Data<Database>, request: Json<OwnerRequest>) -> HttpResponse {
    match db
        .owner
        .create_owner(
            Owner::try_from(OwnerRequest {
                name: request.name.clone(),
                email: request.email.clone(),
                phone: request.phone.clone(),
                address: request.address.clone()
            })
            .expect("Failed to convert OwnerRequest to Owner")
        )
        .await {
        Ok(result) => HttpResponse::Created().json(result),
        Err(e) => {
            eprintln!("Error creating owner: {}", e);
            HttpResponse::InternalServerError().body("Failed to create owner")
        }
    }
}