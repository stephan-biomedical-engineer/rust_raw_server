use sqlx::{PgPool, Postgres};

use crate::models::user::User;

pub async fn find_all(pool: &PgPool) 
    -> Result<Vec<User>, sqlx::Error>
{
    sqlx::query_as::<Postgres, User>
        (
            "SELECT id, name FROM users ORDER BY id"
        )
        .fetch_all(pool)
        .await
}

pub async fn create(pool: &PgPool, name: String,) 
    -> Result<User, sqlx::Error>
{
    sqlx::query_as::<Postgres, User>
        (
            "INSERT INTO users (name) VALUES ($1) RETURNING id, name"
        )
        .bind(name)
        .fetch_one(pool)
        .await
}

pub async fn find_by_id(pool: &PgPool, id: i32) 
    -> Result<Option<User>, sqlx::Error> 
{
    sqlx::query_as::<Postgres, User>("SELECT id, name FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn update(pool: &PgPool, id: i32, name: String) 
    -> Result<Option<User>, sqlx::Error> 
{
    sqlx::query_as::<Postgres, User>("UPDATE users SET name = $1 WHERE id = $2 RETURNING id, name")
        .bind(name)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn delete(pool: &PgPool, id: i32) 
    -> Result<u64, sqlx::Error> 
{
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}