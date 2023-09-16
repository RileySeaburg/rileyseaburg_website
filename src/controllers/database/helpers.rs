use crate::models::Post;
use rustyroad::database::Database;

pub async fn get_db_pool() -> Result<sqlx::PgPool, sqlx::Error> {
    let database = Database::get_database_from_rustyroad_toml().unwrap();

    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        database.username, database.password, database.host, database.port, database.name
    );

    let db_pool = sqlx::PgPool::connect(&database_url).await?;
    Ok(db_pool)
}

pub async fn execute_query(
    pool: &sqlx::PgPool,
) -> Result<Vec<(i32, String, String, String, bool, String, String)>, sqlx::Error> {
    let rows_returned: Vec<(i32, String, String, String, bool, String, String)> =
        sqlx::query_as("SELECT * FROM Posts")
            .fetch_all(pool)
            .await?;
    Ok(rows_returned)
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_get_db_pool() {
        dotenv().ok();
        let pool = get_db_pool().await.unwrap();
        assert!(pool.acquire().await.is_ok());
    }

    #[tokio::test]
    async fn test_execute_query() {
        dotenv().ok();
        let pool = get_db_pool().await.unwrap();
        let rows = execute_query(&pool).await.unwrap();
        assert!(!rows.is_empty());

        // Assuming known_row_data is the data that should be returned by the query
        // let known_row_data = (id, title, body, author, published, created_at, updated_at);
        // assert_eq!(rows[0], known_row_data);
    }
}
