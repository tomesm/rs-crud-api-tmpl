use super::{Passenger, PassengerDao, PassengerPatch};
use crate::model;
use crate::model::db::init_db;
use crate::security::utx_from_token;

#[tokio::test]
async fn model_passenger_create() -> Result<(), Box<dyn std::error::Error>> {
    // FIXTURE
    let db = init_db().await?;
    let utx = utx_from_token(&db, "f7a25ba8-fc87-4b6f-9297-611921ef0d7a").await?;
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
    assert_eq!(passenger_created.uid.to_string(), utx.user_id);

    Ok(())
}

#[tokio::test]
async fn model_passenger_get_ok() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;
    let utx = utx_from_token(&db, "f7a25ba8-fc87-4b6f-9297-611921ef0d7a").await?;
    // -- ACTION
    let passenger = PassengerDao::get(&db, &utx, "b03535ad-0b98-4c8f-8b5a-66960c71392c".to_string()).await?;
    // -- CHECK
    assert_eq!("b03535ad-0b98-4c8f-8b5a-66960c71392c", passenger.id.to_string());
    assert_eq!("Passenger 101", passenger.first_name);
    Ok(())
}

#[tokio::test]
async fn model_passenger_update_ok() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;
    let utx = utx_from_token(&db, "73b88743-0c2a-4d2c-9b43-a71a582cfbc5").await?;
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
    let passenger_updated =
        PassengerDao::update(&db, &utx, passenger_fx.id.to_string(), update_data_fx.clone()).await?;
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
    let utx = utx_from_token(&db, "f7a25ba8-fc87-4b6f-9297-611921ef0d7a").await?;

    // -- ACTION
    let result = PassengerDao::get(&db, &utx, "52188bd6-733a-4856-a10e-c59b937bb573".to_string()).await;
    // println!("\n\n->> {:?}", result);
    // -- CHECK
    match result {
        Ok(_) => assert!(false, "Should not succeed"),
        Err(model::Error::EntityNotFound(typ, id)) => {
            assert_eq!("passenger", typ);
            assert_eq!("52188bd6-733a-4856-a10e-c59b937bb573", id);
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
    assert_eq!("4208b168-08b2-4c45-915d-c51f6f71213b", passengers[0].id.to_string());
    assert_eq!("4464cab1-74da-45c1-bcec-d9e668175ec0", passengers[0].uid.to_string());
    assert_eq!("Passenger 100", passengers[0].first_name);
    assert_eq!("100", passengers[0].last_name);
    assert!(passengers[0].status == Some("new".to_string()));

    // Passenger 2
    assert_eq!("b03535ad-0b98-4c8f-8b5a-66960c71392c", passengers[1].id.to_string());
    assert_eq!("7bb0d513-6c69-49bb-9b1f-9bf456467f88", passengers[1].uid.to_string());
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
    let passenger = PassengerDao::delete(&db, &utx, "b03535ad-0b98-4c8f-8b5a-66960c71392c".to_string()).await?;
    // -- CHECK - deleted item
    assert_eq!("b03535ad-0b98-4c8f-8b5a-66960c71392c", passenger.id.to_string());
    assert_eq!("Passenger 101", passenger.first_name);
    // -- CHECK - list
    let todos: Vec<Passenger> = PassengerDao::list(&db, &utx).await?;
    assert_eq!(1, todos.len());
    Ok(())
}
