use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,

    #[serde(skip_serializing)] 
    pub password_hash: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest
{
    pub name: String,
}