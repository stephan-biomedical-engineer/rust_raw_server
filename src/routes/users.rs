use serde::Serialize;

use crate::http::response::Response;

#[derive(Serialize)]
struct User
{
    id: u32,
    name: String,
}

pub fn list_users() -> Response
{
    let users = vec!
    [
        User
        {
            id: 1,
            name: "Stephan".to_string(),
        },
        User
        {
            id: 2,
            name: "Maria".to_string(),
        },
    ];

    let json = serde_json::to_string(&users)
      .expect("[ERROR] Failed to serialize users");

    Response::json(200, "OK", &json)
}