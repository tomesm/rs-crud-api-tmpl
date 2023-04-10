use super::init_db;

#[tokio::test]
async fn model_db_init_db() -> Result<(), Box<dyn std::error::Error>> {
    // ACTION
    let _db = init_db().await?;

    // CHECK
    let result = sqlx::query("SELECT * FROM passenger").fetch_all(&_db).await?;
    assert_eq!(2, result.len(), "Expected 2 passengers in db");
    Ok(())
}
