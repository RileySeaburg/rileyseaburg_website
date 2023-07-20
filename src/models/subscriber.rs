use serde::{Deserialize, Serialize};
use sqlx::{PgConnection, Pool, Postgres};

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

        sqlx::query!("INSERT INTO Subscriber (email) VALUES ($1)", self.email)
            .execute(conn)
            .await?;

        Ok(())
    }
}

pub struct NewsletterForm {
    pub email: String,
}
