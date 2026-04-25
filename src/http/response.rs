pub struct Response
{
    pub status_code: u16,
    pub status_text: String,
    pub content_type: String,
    pub body: String,
}

impl Response
{
    pub fn new(status_code: u16, status_text: &str, body: &str) -> Response
    {
        Response
        {
            status_code,
            status_text: status_text.to_string(),
            content_type: "text/html; charset=UTF-8".to_string(),
            body: body.to_string(),
        }
    }

    pub fn ok(body: &str) -> Response
    {
        Response::new(200, "OK", body)
    }

    pub fn not_found() -> Response
    {
        Response::new(404, "NOT FOUND", "<h1>404</h1><p>Página não encontrada.</p>")
    }

    pub fn to_http_string(&self) -> String
    {
        format!
        (
            "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
            self.status_code,
            self.status_text,
            self.content_type,
            self.body.as_bytes().len(),
            self.body
        )
    }

    pub fn json(status_code: u16, status_text: &str, body: &str) -> Response
    {
        Response
        {
            status_code,
            status_text: status_text.to_string(),
            content_type: "application/json; charset=UTF-8".to_string(),
            body: body.to_string(),
        }
    }
}