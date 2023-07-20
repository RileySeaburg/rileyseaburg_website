use rustyroad::database::{Database, DatabaseConnection};
use serde::{Deserialize, Serialize};
use sqlx::{Executor, MySql, Pool, Postgres, Sqlite};

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscriber {
    pub id: i32,
    pub email: String,
}

impl Subscriber {
    pub fn new(email: &str) -> Self {
        Self {
            id: 0,
            email: email.to_string(),
        }
    }

    /// # Name: insert
    /// ## Description
    /// Inserts a new subscriber into the database.
    /// ## Parameters
    /// - `conn` - A mutable reference to a database connection.
    /// ## Returns
    /// - `Result<(), sqlx::Error>` - A result that returns nothing if successful, or an error if unsuccessful.
    /// ## Example
    /// ```rust
    /// use rustyroad::database::Database;
    /// use sqlx::PgPool;
    /// use crate::models::Subscriber;
    ///     
    /// #[actix_web::main]
    /// async fn main() {
    ///    let database = Database::get_database_from_rustyroad_toml().unwrap();
    ///   let database_url = format!(
    ///       "postgres://{}:{}@{}:{}/{}",
    ///      database.username,
    ///     database.password,
    ///    database.host,
    ///   database.port,
    /// database.name
    /// );
    /// let mut conn = PgPool::connect(&database_url)
    ///    .await
    ///  .expect("Failed to connect to Postgres.");
    /// let new_subscriber = Subscriber::new("test@test");
    /// let result = new_subscriber.insert(&mut conn).await;
    /// assert!(result.is_ok());
    /// }
    /// ```
    /// ## Panics
    /// This function will panic if the database connection fails.
    /// ## Errors
    /// This function will return an error if the database connection fails.
    pub async fn insert(&self, conn: &mut Pool<Postgres>) -> Result<(), sqlx::Error> {
        let database: Database = Database::get_database_from_rustyroad_toml().expect("Couldn't parse the rustyroad.toml file. Please check the documentation for a proper implementation.");
        let sql = format!("INSERT INTO Subscriber (email) VALUES ('{}')", self.email);
        let connection = Database::create_database_connection(&database)
            .await
            .unwrap_or_else(|why| {
                panic!("Couldn't create database connection: {}", why.to_string())
            });

        match connection.clone() {
            DatabaseConnection::Pg(connection) => {
                println!("Executing query: {:?}", sql.clone().as_str());
                //unwrap the arc
                let rows_affected = connection.execute(sql.as_str()).await?;
                println!("{:?} rows affected", rows_affected);
            }
            DatabaseConnection::MySql(connection) => {
                println!("Executing query: {:?}", sql);
                let rows_affected = connection.execute(sql.as_str()).await?;
                println!("{:?} rows affected", rows_affected);
            }
            DatabaseConnection::Sqlite(connection) => {
                println!("Executing query: {:?}", sql);
                let rows_affected = connection.execute(sql.as_str()).await?;
                println!("{:?} rows affected", rows_affected);
            }
        };


        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewsletterForm {
    pub email: String,
}
