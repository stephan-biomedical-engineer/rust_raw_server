use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct User 
{
    pub id: i32,
    pub name: String,
    pub email: String,

    #[serde(skip_serializing)] 
    pub password_hash: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserRequest
{
    #[validate(length(min = 2, max = 100))]
    pub name: String,
}