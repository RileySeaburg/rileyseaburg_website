use rustyroad::database::Database;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub author: String,
    pub created_at: String,
    pub updated_at: String,

    pub published: bool,
}

impl Post {
    pub async fn get_all_blogs() -> Result<Vec<Post>, sqlx::Error> {
        let mut blogs: Vec<Post> = Vec::new();
        let mut blog: Post;
        let mut blog_row: (i32, String, String, String, bool, String, String);
        let blog_rows = sqlx::query_as("SELECT * FROM Posts");

        let database = Database::get_database_from_rustyroad_toml().unwrap();

        // Create the database URL
        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            database.username, database.password, database.host, database.port, database.name
        );

        // Create the database connection pool
        let db_pool = sqlx::PgPool::connect(&database_url)
            .await
            .expect("Failed to connect to Postgres.");

        let rows_returned: Vec<(i32, String, String, String, bool, String, String)> = blog_rows
            .fetch_all(&db_pool.clone())
            .await
            .unwrap_or_else(|_| panic!("Failed to fetch all blogs from the database."));

        for i in 0..rows_returned.len() {
            blog_row = rows_returned[i].clone();
            blog = Post {
                id: blog_row.0.clone(),
                title: blog_row.1.clone(),
                body: blog_row.2.clone(),
                author: blog_row.3.clone(),
                published: blog_row.4.clone(),
                created_at: blog_row.5.clone(),
                updated_at: blog_row.6.clone(),
            };
            blogs.push(blog);
        }

        Ok(blogs)
    }
}
