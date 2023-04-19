use super::handlers;
use crate::model::{init_db, Passenger, PassengerDao};
use crate::security::utx_from_token;
use crate::web::handle_rejection;
use anyhow::{Context, Result};
use serde::Deserialize;
use serde_json::{from_str, from_value, json, Value};
use std::{str::from_utf8, sync::Arc};
use warp::hyper::body::Bytes;
use warp::hyper::Response;
use warp::Filter;

#[tokio::test]
async fn web_handlers_list() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let passenger_apis = handlers("api", db.clone()).recover(handle_rejection);
    // -- ACTION
    let response = warp::test::request()
        .method("GET")
        .header("X-Auth-Token", "3cb430d0-8914-4c71-aaf9-0ed2b163eca6")
        .path("/api/passengers")
        .reply(&passenger_apis)
        .await;
    // -- CHECK
    assert_eq!(response.status(), 200, "http status");
    let passengers: Vec<Passenger> = extract_body_data(response)?;
    // -- CHECK - passengers
    assert_eq!(2, passengers.len(), "number of passengers");
    assert_eq!("4208b168-08b2-4c45-915d-c51f6f71213b", passengers[0].id.to_string());
    assert_eq!("Passenger 100", passengers[0].first_name);

    Ok(())
}

#[tokio::test]
async fn web_passenger_get_ok() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let passsenger_apis = handlers("api", db).recover(handle_rejection);

    // -- ACTION
    let resp = warp::test::request()
        .method("GET")
        .header("X-Auth-Token", "3cb430d0-8914-4c71-aaf9-0ed2b163eca6")
        .path("/api/passengers/4208b168-08b2-4c45-915d-c51f6f71213b")
        .reply(&passsenger_apis)
        .await;

    // -- CHECK - status
    assert_eq!(200, resp.status(), "http status");

    // extract response .data
    let passenger: Passenger = extract_body_data(resp)?;

    // -- CHECK - .data (todo)
    assert_eq!("4208b168-08b2-4c45-915d-c51f6f71213b", passenger.id.to_string());
    assert_eq!("Passenger 100", passenger.first_name);
    assert_eq!("100", passenger.last_name);
    assert_eq!(Some("new".to_string()), passenger.status);

    Ok(())
}

#[tokio::test]
async fn web_passenger_create_ok() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let passenger_apis = handlers("api", db.clone()).recover(handle_rejection);
    // new todo fixture
    const STATUS: &str = "test - web_passenger_create_ok";
    let body = json!({
        "status": STATUS,
    });

    // -- ACTION
    let resp = warp::test::request()
        .method("POST")
        .header("X-Auth-Token", "3cb430d0-8914-4c71-aaf9-0ed2b163eca6")
        .path("/api/passengers")
        .json(&body)
        .reply(&passenger_apis)
        .await;

    // -- CHECK - status
    assert_eq!(200, resp.status(), "http status");
    // extract response .data
    let passenger: Passenger = extract_body_data(resp)?;
    // -- CHECK - .data (passenger)
    assert_eq!(Some(STATUS.to_string()), passenger.status);

    Ok(())
}

#[tokio::test]
async fn web_passenger_update_ok() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let passenger_apis = handlers("api", db.clone()).recover(handle_rejection);
    // udpated passenger
    const STATUS: &str = "test - passenger 100 updated";
    let body = json!({
        "first_name": "James Tiberius",
        "status": STATUS
    });
    // -- ACTION
    let resp = warp::test::request()
        .method("PATCH")
        .header("X-Auth-Token", "3cb430d0-8914-4c71-aaf9-0ed2b163eca6")
        .path("/api/passengers/4208b168-08b2-4c45-915d-c51f6f71213b")
        .json(&body)
        .reply(&passenger_apis)
        .await;

    // -- CHECK - status
    assert_eq!(200, resp.status(), "http status");

    // extract response .data
    let passenger: Passenger = extract_body_data(resp)?;

    // -- CHECK - .data (todo)
    assert_eq!(
        "4208b168-08b2-4c45-915d-c51f6f71213b",
        passenger.id.to_string(),
        "passenger.id"
    );
    assert_eq!("James Tiberius", passenger.first_name);
    assert_eq!(Some(STATUS.to_string()), passenger.status);

    Ok(())
}

#[tokio::test]
async fn web_todo_delete_ok() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let passengers_apis = handlers("api", db.clone()).recover(handle_rejection);

    // -- ACTION
    let resp = warp::test::request()
        .method("DELETE")
        .header("X-Auth-Token", "3cb430d0-8914-4c71-aaf9-0ed2b163eca6")
        .path("/api/passengers/4208b168-08b2-4c45-915d-c51f6f71213b")
        .reply(&passengers_apis)
        .await;

    // -- CHECK - status
    assert_eq!(200, resp.status(), "http status");

    // extract response .data
    let passenger: Passenger = extract_body_data(resp)?;
    // println!("\n\n->> {:?}", passengers);

    // -- CHECK - .data (todos)
    assert_eq!("4208b168-08b2-4c45-915d-c51f6f71213b", passenger.id.to_string());
    assert_eq!("Passenger 100", passenger.first_name);

    // -- CHECK - list .len() should be 1
    let utx = utx_from_token(&db, "3cb430d0-8914-4c71-aaf9-0ed2b163eca6").await?;
    let passengers = PassengerDao::list(&db, &utx).await?;
    assert_eq!(1, passengers.len(), "passengers length");
    assert_eq!(
        "b03535ad-0b98-4c8f-8b5a-66960c71392c",
        passengers[0].id.to_string(),
        "Passenger remaining should be 101"
    );

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
