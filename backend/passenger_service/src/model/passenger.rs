use std::fmt;
use std::str::FromStr;

use super::db::Db;
use crate::model;
use crate::security::UserCtx;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use sqlbuilder::SqlBuilder;
use sqlx::types::Uuid;
use utoipa::ToSchema;

// region: use  Passenger Types
#[serde_as]
#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Passenger {
    #[schema(example = "4208b168-08b2-4c45-915d-c51f6f71213b")]
    #[serde_as(as = "DisplayFromStr")]
    pub id: Uuid,
    #[schema(example = "2096036b-9606-4405-995b-565a481344bc")]
    #[serde_as(as = "DisplayFromStr")]
    pub uid: Uuid,
    #[schema(example = "John")]
    pub first_name: String,
    #[schema(example = "Doe")]
    pub last_name: String,
    #[schema(example = "new")]
    pub status: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Status {
    Active,
    Inactive,
    Pending(usize),
}

impl FromStr for Status {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Status::Active),
            "inactive" => Ok(Status::Inactive),
            _ => {
                let parts: Vec<_> = s.split(',').collect();
                if parts.len() == 2 && parts[0] == "pending" {
                    if let Ok(value) = parts[1].parse::<usize>() {
                        return Ok(Status::Pending(value));
                    }
                }
                Err(())
            }
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Active => write!(f, "active"),
            Status::Inactive => write!(f, "inactive"),
            Status::Pending(value) => write!(f, "pending,{}", value),
        }
    }
}

#[derive(Default, Clone, Debug, Deserialize)]
pub struct PassengerPatch {
    pub uid: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub status: Option<String>,
}

impl PassengerPatch {
    pub fn get_first_name(&self) -> String {
        self.first_name.clone().unwrap_or_else(|| "untitled".to_string())
    }

    pub fn get_last_name(&self) -> String {
        self.last_name.clone().unwrap_or_else(|| "untitled".to_string())
    }

    pub fn get_status(&self) -> String {
        self.status.clone().unwrap_or_else(|| "new".to_string())
    }
}

// endregion:  Passenger Types

// region: PassengerMac (Model Access Controller)
pub struct PassengerDao;

impl PassengerDao {
    const TABLE: &'static str = "passenger";
    const COLUMNS: &'static [&'static str] = &["uid", "first_name", "last_name", "status"];
}

impl PassengerDao {
    pub async fn create(db: &Db, utx: &UserCtx, data: PassengerPatch) -> Result<Passenger, model::Error> {
        let sql = SqlBuilder::new()
            .insert_into(Self::TABLE) // Start the INSERT statement and specify the table name
            .columns(Self::COLUMNS) // Specify the columns to insert into
            .values(&[
                &utx.user_id.to_string(),
                &data.get_first_name(),
                &data.get_last_name(),
                &data.get_status(),
            ])
            .build();
        let query = sqlx::query_as::<_, Passenger>(&sql);
        let passenger = query.fetch_one(db).await?;
        Ok(passenger)
    }

    // Create from bulk (array of PassengerPatch)
    // pub async fn create_bulk(
    //     db: &Db,
    //     utx: &UserCtx,
    //     data: Vec<PassengerPatch>,
    // ) -> Result<Vec<Passenger>, model::Error> {
    //     let mut passengers: Vec<Passenger> = Vec::new();
    //     for passenger in data {
    //         let sql = SqlBuilder::new()
    //             .insert_into(Self::TABLE) // Start the INSERT statement and specify the table name
    //             .columns(Self::COLUMNS) // Specify the columns to insert into
    //             .values(&[
    //                 &utx.user_id.to_string(),
    //                 &passenger.get_first_name(),
    //                 &passenger.get_last_name(),
    //                 &passenger.get_status(),
    //             ])
    //             .build();
    //         let query = sqlx::query_as::<_, (Passenger)>(&sql);
    //         let passenger = query.fetch_one(db).await?;
    //         passengers.push(passenger);
    //     }
    //     Ok(passengers)
    // }

    pub async fn get(db: &Db, _utx: &UserCtx, id: String) -> Result<Passenger, model::Error> {
        let sql = SqlBuilder::new()
            .select_from(Self::TABLE)
            .where_clause("id = {}", id.clone())
            .build();
        let query = sqlx::query_as::<_, Passenger>(&sql);
        let result = query.fetch_one(db).await;
        handle_fetch_one_result(result, Self::TABLE, id)
    }

    pub async fn update(db: &Db, utx: &UserCtx, id: String, data: PassengerPatch) -> Result<Passenger, model::Error> {
        let sql = SqlBuilder::new()
            .update(Self::TABLE)
            .set_columns_and_values(
                Self::COLUMNS,
                &[
                    &utx.user_id.to_string(),
                    &data.get_first_name(),
                    &data.get_last_name(),
                    &data.get_status(),
                ],
            )
            .where_clause("id = {}", id.clone())
            .build();
        let query = sqlx::query_as::<_, Passenger>(&sql);
        let result = query.fetch_one(db).await;
        handle_fetch_one_result(result, Self::TABLE, id)
    }

    pub async fn delete(db: &Db, _utx: &UserCtx, id: String) -> Result<Passenger, model::Error> {
        let sql = SqlBuilder::new()
            .delete_from(Self::TABLE)
            .where_clause("id = {}", id.clone())
            .build();
        let query = sqlx::query_as::<_, Passenger>(&sql);
        let result = query.fetch_one(db).await;
        handle_fetch_one_result(result, Self::TABLE, id)
    }

    pub async fn list(db: &Db, _utx: &UserCtx) -> Result<Vec<Passenger>, model::Error> {
        let sql = SqlBuilder::new().select_from(Self::TABLE).order_by("id").build();
        let query = sqlx::query_as(&sql);
        let passengers = query.fetch_all(db).await?;
        Ok(passengers)
    }
}

// endregion: PassengerMac (Model Access Controller)

// region:    Utils
fn handle_fetch_one_result(
    result: Result<Passenger, sqlx::Error>,
    typ: &'static str,
    id: String,
) -> Result<Passenger, model::Error> {
    result.map_err(|sqlx_error| match sqlx_error {
        sqlx::Error::RowNotFound => model::Error::EntityNotFound(typ, id.to_string()),
        other => model::Error::SqlxError(other),
    })
}
// endregion: Utils

#[cfg(test)]
#[path = "../_tests/model_passenger.rs"]
mod tests;
