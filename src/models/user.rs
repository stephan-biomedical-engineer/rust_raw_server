use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct User 
{
    pub id: i32,
    pub name: String,
    // pub email: String,
    // pub password_hash: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest
{
    pub name: String,
    // pub email: String,
    // pub password: String,
}