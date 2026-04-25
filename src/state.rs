use crate::routes::users::User;

#[derive(Debug)]
pub struct AppState
{
    pub users: Vec<User>,
    pub next_user_id: u32,
}

impl AppState
{
    pub fn new() -> AppState
    {
        AppState
        {
            users: Vec::new(),
            next_user_id: 1,
        }
    }
}