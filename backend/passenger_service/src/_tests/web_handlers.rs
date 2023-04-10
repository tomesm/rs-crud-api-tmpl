use super::handlers;
use crate::model::{init_db, Passenger, PassengerDao};
use anyhow::{Context, Result};
use serde::Deserialize;
use serde_json::{from_str, from_value, Value};
use std::{str::from_utf8, sync::Arc};
use warp::hyper::body::Bytes;
use warp::hyper::Response;

#[tokio::test]
async fn web_handlers_list() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let passenger_apis = handlers("api", db.clone());
    // -- ACTION
    let response = warp::test::request()
        .method("GET")
        .header("X-Auth-Token", "123")
        .path("/api/passengers")
        .reply(&passenger_apis)
        .await;
    // -- CHECK
    assert_eq!(response.status(), 200, "http status");
    let passengers: Vec<Passenger> = extract_body_data(response)?;
    // -- CHECK - passengers
    assert_eq!(2, passengers.len(), "number of passengers");
    assert_eq!(100, passengers[0].id);
    assert_eq!("Passenger 100", passengers[0].first_name);

    Ok(())
}

// region: Web Test Utils
fn extract_body_data<D>(resp: Response<Bytes>) -> Result<D>
where
    for<'de> D: Deserialize<'de>,
{
    // parse the body as serde_json::Value
    let body = from_utf8(resp.body())?;
    let mut body: Value =
        from_str(body).with_context(|| format!("Cannot parse resp.body to JSON. resp.body: '{}'", body))?;
    // extract the data
    let data = body["data"].take();
    // deserialize the data to D
    let data: D = from_value(data)?;
    Ok(data)
}

//
