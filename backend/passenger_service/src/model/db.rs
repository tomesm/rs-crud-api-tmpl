#![allow(dead_code)]
#![allow(unused)]

use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

pub type Db = Pool<Postgres>;

// region db connection constants
const PG_HOST: &str = "0.0.0.0";
const PG_PORT_DB: &str = "26257";
const PG_ROOT_DB: &str = "ims";
const PG_ROOT_USER: &str = "root";
const PG_ROOT_PWD: &str = "";
const PG_SSL_MODE: &str = "disable";
// app db
const PG_APP_DB: &str = "passenger_service_db";
const PG_APP_USER: &str = "passenger_service_user";
const PG_APP_PWD: &str = "passenger_service_pwd_to_change";
const PG_APP_MAX_CONN: u32 = 5;
// sql files
const SQL_DIR: &str = "sql/";
const SQL_RECREATE: &str = "sql/00-recreate-db.sql";
// endregion db connection constants

pub async fn init_db() -> Result<Db, sqlx::Error> {
    // Create the db with PG_ROOT (dev only)
    {
        let root_db = new_db_pool(
            PG_HOST,
            PG_ROOT_DB,
            PG_ROOT_USER,
            PG_PORT_DB,
            PG_ROOT_PWD,
            PG_SSL_MODE,
            1,
        )
        .await?;
        pexec(&root_db, SQL_RECREATE).await?;
    }
    // Run the app sql files
    let app_db = new_db_pool(PG_HOST, PG_APP_DB, PG_APP_USER, PG_PORT_DB, PG_APP_PWD, PG_SSL_MODE, 1).await?;
    let mut paths: Vec<PathBuf> = fs::read_dir(SQL_DIR)?
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .collect();
    paths.sort();
    for path in paths {
        if let Some(path) = path.to_str() {
            // only sql files and not the recreate
            if path.ends_with(".sql") && path != SQL_RECREATE {
                pexec(&app_db, &path).await?;
            }
        }
    }
    // return app db
    new_db_pool(
        PG_HOST,
        PG_APP_DB,
        PG_APP_USER,
        PG_PORT_DB,
        PG_APP_PWD,
        PG_SSL_MODE,
        PG_APP_MAX_CONN,
    )
    .await
}

async fn pexec(db: &Db, file: &str) -> Result<(), sqlx::Error> {
    // Read the file
    let content = fs::read_to_string(file).map_err(|ex| {
        println!("ERROR reading file: {} (cause: {:?}", file, ex);
        ex
    })?;
    // TODO: make the split more sql proof
    let sqls: Vec<&str> = content.split(";").collect();
    for sql in sqls {
        match sqlx::query(&sql).execute(db).await {
            Ok(_) => {}
            Err(ex) => {
                println!("WARNING - pexec - Sql file '{}' FAILED cause: {}", file, ex);
            }
        }
    }
    Ok(())
}

async fn new_db_pool(
    host: &str,
    db: &str,
    user: &str,
    port: &str,
    pwd: &str,
    sslmode: &str,
    max_con: u32,
) -> Result<Db, sqlx::Error> {
    let db_url = format!(
        "postgresql://{}:{}@{}:{}/{}?sslmode={}",
        user, pwd, host, port, db, sslmode
    );

    PgPoolOptions::new()
        .max_connections(max_con)
        .connect_timeout(Duration::from_millis(500))
        .connect(&db_url)
        .await
}

#[cfg(test)]
#[path = "../_tests/model_db.rs"]
mod tests;
