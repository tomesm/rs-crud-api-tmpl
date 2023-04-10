use super::filter_auth::do_auth;
use super::filter_utils::with_db;
use crate::{
    model::{Db, Passenger, PassengerDao},
    security::{utx_from_token, UserCtx},
};
use serde_json::json;
use std::sync::Arc;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use warp::reply::Json;
use warp::Filter;

pub fn handlers(
    base_path: &'static str,
    db: Arc<Db>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let passengers_path = warp::path(base_path).and(warp::path("passengers"));
    // Each of our routes will have its own copy of the db Arc.
    let common = with_db(db.clone()).and(do_auth(db.clone()));

    // LIST passengers `GET /passengers`
    let list = passengers_path
        .and(warp::get())
        .and(warp::path::end())
        .and(common.clone())
        .and_then(list_passengers);
    list
}

#[utoipa::path(
    get,
    path = "/api/passengers",
    params (
        ("X-Auth-Token" = String, description = "Authentication token"),
    ),
    responses (
        (status = 200, description = "List of passengers", body = [Passenger]),
    )
)]
pub async fn list_passengers(db: Arc<Db>, utx: UserCtx) -> Result<Json, warp::Rejection> {
    // FIXME: Add proper error handling
    let passengers = PassengerDao::list(&db, &utx).await?;
    let response = json!({ "data": passengers });
    Ok(warp::reply::json(&response))
}

// region:    Tests
#[cfg(test)]
#[path = "../_tests/web_handlers.rs"]
mod tests;

// endregion: Tests
