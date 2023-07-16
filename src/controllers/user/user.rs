use crate::models::User;

/// # Name: get_user_from_session_token
/// 
/// ## Description
/// 
/// This function takes a session token and a database connection pool and returns an optional user.
/// 
/// ## Arguments
/// 
/// * `session_token` - A string slice that holds the session token.
/// * `pool` - A reference to a database connection pool.
/// 
/// ## Returns
/// 
/// An optional user.
/// 
/// ## Example
/// 
/// ```
/// use rustyroad::database::Database;
/// 
/// let database = Database {
///    username: "postgres".to_string(),
///   password: "postgres".to_string(),
///   host: "localhost".to_string(),
///  port: 5432,
/// name: "example".to_string(),
/// };
/// 
/// let database_url = format!(
///    "postgres://{}:{}@{}:{}/{}",
///   database.username, database.password, database.host, database.port, database.name
/// );
/// 
/// let db_pool = PgPool::connect(&database_url)
///   .await
///  .expect("Failed to connect to Postgres.");
/// d
/// let user = get_user_from_session_token("session_token", &db_pool).await;
/// 
/// assert_eq!(user.is_some(), true);
/// ```
/// 
pub async fn get_user_from_session_token(session_token: &str, pool: &sqlx::PgPool) -> Option<User> {
    let user: Result<User, sqlx::Error> = sqlx::query_as(
        "
        SELECT Users.id, Users.username, Users.password, Users.email, Users.created_at, Users.updated_at 
        FROM Users 
        INNER JOIN Sessions ON Users.id = Sessions.user_id
        WHERE Sessions.session_token = $1 AND Sessions.expiration_date > NOW()
        ",
    )
    .bind(session_token)
    .fetch_one(pool)
    .await;

    match user {
        Ok(user) => Some(user),
        Err(_) => None,
    }
}
