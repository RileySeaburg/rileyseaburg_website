use rustyroad::database::{Database, DatabaseConnection};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

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

pub struct NewsletterForm {
    pub email: String,
}
