use super::filter_auth::do_auth;
use super::filter_utils::with_db;
use crate::{
    model::{Db, PassengerDao, PassengerPatch},
    security::UserCtx,
};
use serde::Serialize;
use serde_json::json;
use std::sync::Arc;
use warp::reply::Json;
use warp::Filter;

pub fn handlers(
    base_path: &'static str,
    db: Arc<Db>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let passengers_path = warp::path(base_path).and(warp::path("passengers"));
    // Each of our routes will have its own copy of the db Arc.
    let common = with_db(db.clone()).and(do_auth(db.clone()));

    let list = passengers_path
        .and(warp::get())
        .and(warp::path::end())
        .and(common.clone())
        .and_then(list_passengers);

    let get = passengers_path
        .and(warp::get())
        .and(common.clone())
        .and(warp::path::param())
        .and_then(get_passenger);

    let create = passengers_path
        .and(warp::post())
        .and(common.clone())
        .and(warp::body::json())
        .and_then(create_passenger);

    let update = passengers_path
        .and(warp::patch())
        .and(common.clone())
        .and(warp::path::param())
        .and(warp::body::json())
        .and_then(update_passenger);

    let delete = passengers_path
        .and(warp::delete())
        .and(common.clone())
        .and(warp::path::param())
        .and_then(delete_passenger);

    list.or(get).or(create).or(update).or(delete)
}

/// List passengers
///
// region: Swagger LIST passengers `GET /passengers`
#[utoipa::path(
    get,
    path = "/api/passengers",
    params (
        ("X-Auth-Token" = String, Header, description = "Authentication token"),
    ),
    responses (
        (status = 200, description = "List of passengers", body = [Passenger]),
    )
)]
// endregion: Swagger LIST passengers `GET /passengers`
pub async fn list_passengers(db: Arc<Db>, utx: UserCtx) -> Result<Json, warp::Rejection> {
    // FIXME: Add proper error handling
    let passengers = PassengerDao::list(&db, &utx).await?;
    json_response(passengers)
}

/// Get passenger
///
// region: Swagger GET passenger `GET /passengers/100`
#[utoipa::path(
    get,
    path = "/api/passengers/{id}",
    params (
        ("id" = String, Path, description = "Passenger's UUID"),
        ("X-Auth-Token" = String, Header, description = "Authentication token"),
    ),
    responses(
        (status = 200, description = "Delete successful", body = Passenger),
        (status = 400, description = "Missing Auth Token request header"),
        (status = 401, description = "Unauthorized to fetch a passenger"),
        (status = 404, description = "Passenger not found"),
    )
)]
// endregion: Swagger GET passenger `GET /passengers/100`
async fn get_passenger(db: Arc<Db>, utx: UserCtx, id: String) -> Result<Json, warp::Rejection> {
    let passenger = PassengerDao::get(&db, &utx, id).await?;
    json_response(passenger)
}

/// Create passenger
///
// region: CREATE passenger `POST /passengers with body PassengerPatch`
#[utoipa::path(
    post,
    path = "/api/passengers",
    params (
        ("X-Auth-Token" = String, Header, description = "Authentication token"),
    ),
    request_body=Passenger,
    responses(
        (status = 200, description = "Passenger created successfully", body = Passenger),
        (status = 409, description = "Passenger already exists")
    )
)]
// endregion: CREATE passenger `POST /passengers with body PassengerPatch`
async fn create_passenger(db: Arc<Db>, utx: UserCtx, patch: PassengerPatch) -> Result<Json, warp::Rejection> {
    let passenger = PassengerDao::create(&db, &utx, patch).await?;
    json_response(passenger)
}

/// Update passenger
///
// region: UPDATE passenger `PATCH /passengers/100 with body PassengerPatch`
#[utoipa::path(
    patch,
    path = "/api/passengers/{id}",
    params (
        ("id" = String, Path, description = "Passenger's UUID"),
        ("X-Auth-Token" = String, Header, description = "Authentication token"),
    ),
    request_body=Passenger,
    responses(
        (status = 200, description = "Passenger updated successfully", body = Passenger),
        (status = 400, description = "Missing Auth Token request header"),
        (status = 401, description = "Unauthorized to update a passenger"),
        (status = 404, description = "Passenger not found"),
    )
)]
// endregion: UPDATE passenger `PATCH /passengers/100 with body PassengerPatch`
async fn update_passenger(
    db: Arc<Db>,
    utx: UserCtx,
    id: String,
    patch: PassengerPatch,
) -> Result<Json, warp::Rejection> {
    let passenger = PassengerDao::update(&db, &utx, id, patch).await?;
    json_response(passenger)
}

/// Delete passenger
///
/// Delete a passenger from database
// region: DELETE passenger `DELETE /passengers/100`
#[utoipa::path(
    delete,
    path = "/api/passengers/{id}",
    params (
        ("id" = String, Path, description = "Passenger's UUID"),
        ("X-Auth-Token" = String, Header, description = "Authentication token"),
    ),
    responses(
        (status = 200, description = "Delete successful", body = Passenger),
        (status = 400, description = "Missing Auth Token request header"),
        (status = 401, description = "Unauthorized to delete a passenger"),
        (status = 404, description = "Passenger not found"),
    )
)]
// endregion: DELETE passenger `DELETE /passengers/100`
async fn delete_passenger(db: Arc<Db>, utx: UserCtx, id: String) -> Result<Json, warp::Rejection> {
    let passenger = PassengerDao::delete(&db, &utx, id).await?;
    json_response(passenger)
}

// region: Utils
fn json_response<D: Serialize>(data: D) -> Result<Json, warp::Rejection> {
    let response = json!({ "data": data });
    Ok(warp::reply::json(&response))
}

// endregion: Utils

// region:    Tests
#[cfg(test)]
#[path = "../_tests/web_handlers.rs"]
mod tests;

// endregion: Tests
