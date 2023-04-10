use super::{Passenger, PassengerDao, PassengerPatch};
use crate::model;
use crate::model::db::init_db;
use crate::security::{utx_from_token, UserCtx};

#[tokio::test]
async fn model_passenger_create() -> Result<(), Box<dyn std::error::Error>> {
    // FIXTURE
    let db = init_db().await?;
    let utx = utx_from_token(&db, "125").await?;
    // Data fixture
    let data_fx = PassengerPatch {
        first_name: Some("test - model_passenger_create 1".to_string()),
        ..Default::default()
    };
    // ACTION
    let passenger_created = PassengerDao::create(&db, &utx, data_fx.clone()).await?;
    // CHECK
    // println!("\n\n->> {:?}", passenger_created);
    assert_eq!(passenger_created.first_name, data_fx.first_name.unwrap());
    assert!(passenger_created.id >= 1000);
    assert_eq!(passenger_created.uid, utx.user_id);

    Ok(())
}

#[tokio::test]
async fn model_passenger_get_ok() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;
    let utx = utx_from_token(&db, "123").await?;
    // -- ACTION
    let passenger = PassengerDao::get(&db, &utx, 100).await?;
    // -- CHECK
    assert_eq!(100, passenger.id);
    assert_eq!("Passenger 100", passenger.first_name);
    Ok(())
}

#[tokio::test]
async fn model_passenger_update_ok() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;
    let utx = utx_from_token(&db, "126").await?;
    let data_fx = PassengerPatch {
        first_name: Some("test - model_passenger_update_ok 1".to_string()),
        ..Default::default()
    };
    let passenger_fx = PassengerDao::create(&db, &utx, data_fx.clone()).await?;
    // println!("\n\n->> {:?}", passenger_fx);
    let update_data_fx = PassengerPatch {
        first_name: Some("test - model_passenger_update_ok 2".to_string()),
        ..Default::default()
    };
    // -- ACTION
    let passenger_updated = PassengerDao::update(&db, &utx, passenger_fx.id, update_data_fx.clone()).await?;
    // println!("\n\n->> {:?}", passenger_updated);
    // -- CHECK
    let passengers = PassengerDao::list(&db, &utx).await?;
    assert_eq!(3, passengers.len());
    assert_eq!(passenger_fx.id, passenger_updated.id);
    assert_eq!(update_data_fx.first_name.unwrap(), passenger_updated.first_name);
    Ok(())
}

#[tokio::test]
async fn model_passenger_get_wrong_id() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;
    let utx = utx_from_token(&db, "123").await?;

    // -- ACTION
    let result = PassengerDao::get(&db, &utx, 999).await;
    // println!("\n\n->> {:?}", result);
    // -- CHECK
    match result {
        Ok(_) => assert!(false, "Should not succeed"),
        Err(model::Error::EntityNotFound(typ, id)) => {
            assert_eq!("passenger", typ);
            assert_eq!(999.to_string(), id);
        }
        other_error => assert!(false, "Wrong Error {:?} ", other_error),
    }
    Ok(())
}

#[tokio::test]
async fn model_passenger_list() -> Result<(), Box<dyn std::error::Error>> {
    // ARRANGE/FIXTURE
    let db = init_db().await?;
    let utx = utx_from_token(&db, "125").await?;
    // ACTION
    let passengers = PassengerDao::list(&db, &utx).await?;
    // CHECK
    assert_eq!(2, passengers.len());
    // println!("\n\n->> {:?}", passengers);
    // Passenger 1
    assert_eq!(100, passengers[0].id);
    assert_eq!(123, passengers[0].uid);
    assert_eq!("Passenger 100", passengers[0].first_name);
    assert_eq!("100", passengers[0].last_name);
    assert!(passengers[0].status == Some("new".to_string()));

    // Passenger 2
    assert_eq!(101, passengers[1].id);
    assert_eq!(124, passengers[1].uid);
    assert_eq!("Passenger 101", passengers[1].first_name);
    assert_eq!("101", passengers[1].last_name);
    assert!(passengers[1].status.is_none());
    Ok(())
}

#[tokio::test]
async fn model_passenger_delete_simple() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;
    let utx = utx_from_token(&db, "123").await?;
    // -- ACTION
    let passenger = PassengerDao::delete(&db, &utx, 100).await?;
    // -- CHECK - deleted item
    assert_eq!(100, passenger.id);
    assert_eq!("Passenger 100", passenger.first_name);
    // -- CHECK - list
    let todos: Vec<Passenger> = PassengerDao::list(&db, &utx).await?;
    assert_eq!(1, todos.len());
    Ok(())
}
