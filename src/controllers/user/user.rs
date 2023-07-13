use crate::models::User;

pub async fn get_user_from_session_token(
    session_token: &str,
    pool: &sqlx::PgPool,
) -> Result<User, sqlx::Error> {
    sqlx::query_as(
        "
        SELECT Users.id, Users.username, Users.password, Users.email, Users.created_at, Users.updated_at 
        FROM Users 
        INNER JOIN Sessions ON Users.id = Sessions.user_id
        WHERE Sessions.session_token = $1 AND Sessions.expiration_date > NOW()
        ",
    )
    .bind(session_token)
    .fetch_one(pool)
    .await
}
