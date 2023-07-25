use chrono::NaiveDateTime;
use rustyroad::database::Database;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::controllers::{execute_query, get_db_pool};

use crate::controllers::datetime::{self, datetime as local_datetime};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, FromRow)]
pub struct Post {
    pub id: Option<i32>,
    pub author: Option<String>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub tags: Option<Vec<String>>,
    #[serde(with = "local_datetime")]  // use the custom serializer/deserializer
    pub publish_date: Option<NaiveDateTime>,
    pub status: Option<String>,
    pub image_url: Option<String>,
    pub category: Option<String>,
    #[serde(with = "local_datetime")]  // use the custom serializer/deserializer
    pub created_at: Option<NaiveDateTime>,
    #[serde(with = "local_datetime")]  // use the custom serializer/deserializer
    pub updated_at: Option<NaiveDateTime>,
    pub slug: Option<String>,
}



impl Post {
    /// # Name: get_all_posts
    /// 
    /// # Arguments
    /// - None
    /// 
    /// # Returns
    ///
    /// 
    ///
    /// This function returns a vector of all the posts in the database.
    /// 
    /// # Example
    /// ```rust
    /// use rustyroad::models::post::Post;
    /// 
    /// #[actix_web::get("/posts")]
    /// async fn get_all_posts() -> Result<Vec<Post>, sqlx::Error> {
    ///    let posts = Post::get_all_posts().await?;
    ///   Ok(posts)
    /// }
    /// ```
    pub async fn get_all_posts() -> Result<Vec<Post>, sqlx::Error> {
        let pool: sqlx::PgPool = get_db_pool().await.unwrap();
        let posts: Vec<Post> = sqlx::query_as("SELECT * FROM Posts")
            .fetch_all(&pool)
            .await?;
        Ok(posts)
    }

    /// # Name: get_post_by_slug
    ///
    /// # Arguments
    /// - slug: String
    ///
    /// # Returns
    /// - Post: post
    ///
    /// This function returns a post from the database based on the id.
    ///
    /// # Example
    /// ```rust
    /// use rustyroad::models::post::Post;
    ///
    /// #[actix_web::get("/post/{slug}")]
    /// async fn get_post_by_slug(slug: web::Path<String>) -> Result<Post, sqlx::Error> {
    ///    let post = Post::get_post_by_slug(slug.into_inner()).await?;
    ///  Ok(post)
    /// ```
    pub async fn get_post_slug(slug: String) -> Result<Post, sqlx::Error> {
        let pool: sqlx::PgPool = get_db_pool().await.unwrap();
        let post: Post = sqlx::query_as("SELECT * FROM Posts WHERE slug = $1")
            .bind(slug)
            .fetch_one(&pool)
            .await?;
        Ok(post)
    }


    pub async fn update_post_slug(slug: String, updated_post: Post) -> Result<Post, sqlx::Error> {
        let pool: sqlx::PgPool = get_db_pool().await.unwrap();
        let post: Post = sqlx::query_as("
        UPDATE Posts
        SET author = $1, title = $2, content = $3, tags = $4, publish_date = $5,
            status = $6, image_url = $7, category = $8, updated_at = $9
        WHERE slug = $10
        RETURNING *
    ")
            .bind(updated_post.author)
            .bind(updated_post.title)
            .bind(updated_post.content)
            .bind(updated_post.tags)
            .bind(updated_post.publish_date)
            .bind(updated_post.status)
            .bind(updated_post.image_url)
            .bind(updated_post.category)
            .bind(chrono::Utc::now().naive_utc())
            .bind(slug)
            .fetch_one(&pool)
            .await?;
        Ok(post)
    }


    pub async fn create_post(new_post: Post) -> Result<Post, sqlx::Error> {
        let pool: sqlx::PgPool = get_db_pool().await.unwrap();
        let post: Post = sqlx::query_as("
        INSERT INTO Posts (author, title, content, tags, publish_date, status, image_url, category, created_at, updated_at, slug)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        RETURNING *
    ")
            .bind(new_post.author)
            .bind(new_post.title)
            .bind(new_post.content)
            .bind(new_post.tags)
            .bind(new_post.publish_date)
            .bind(new_post.status)
            .bind(new_post.image_url)
            .bind(new_post.category)
            .bind(new_post.created_at)
            .bind(chrono::Utc::now().naive_utc())
            .bind(new_post.slug)
            .fetch_one(&pool)
            .await?;
        Ok(post)
    }



}


#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;

    #[tokio::test]
    async fn test_get_all_posts() {
        let date_str = "2023-07-23 13:00:00";
        // Set up a test post
        let test_post = Post {
            id: 1,
            author: Some(String::from("John Doe")),
            title: Some(String::from("My First Blog Post")),
            content: Some(String::from("This is the content of my first blog post")),
            tags: Some(vec![String::from("blog"), String::from("first post")]),
            publish_date: Some( NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S").unwrap()),
            status: Some(String::from("published")),
            image_url: Some(String::from("https://images.unsplash.com/photo-1515886657613-9f3515b0c78f?ixlib=rb-1.2.1&auto=format&fit=crop&w=634&q=80")),
            category: Some(String::from("Blog")),
            created_at: Some( NaiveDateTime::parse_from_str("2023-07-23 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap()),
            updated_at: Some( NaiveDateTime::parse_from_str("2023-07-23 13:00:00", "%Y-%m-%d %H:%M:%S").unwrap()),
            slug: Some(String::from("my-first-blog-post")),
        };

        // Test the get_all_posts function
        let result = Post::get_all_posts().await;
        assert!(result.is_ok(), "Failed to get posts from the database");

        let posts = result.unwrap();
        assert!(!posts.is_empty(), "No posts were returned");

        // Find the post with id == 1
        let post_opt = posts.into_iter().find(|p| p.id == 1);
        assert!(post_opt.is_some(), "Post with id == 1 was not found");

        let post = post_opt.unwrap();
        assert_eq!(post, test_post, "Post does not match expected values");
    }
}
