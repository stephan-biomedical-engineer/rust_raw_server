use serde::Serialize;

use crate::http::response::Response;

#[derive(Serialize)]
struct HealthResponse
{
    status: String,
}

pub fn health() -> Response
{
    let body = HealthResponse
    {
        status: "ok".to_string(),
    };

    let json = serde_json::to_string(&body)
      .expect("[ERROR] Failed to serialize health response");

    Response::json(200, "OK", &json)
}