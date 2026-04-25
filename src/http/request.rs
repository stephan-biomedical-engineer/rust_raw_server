#[derive(Debug)]
pub struct Request 
{
    pub method: String,
    pub path: String, 
    pub version: String,
    pub body: String,
}

impl Request
{
    pub fn from_buffer(buffer: &[u8]) -> Option<Request>
    {
        let request_text = String::from_utf8_lossy(buffer);

        let mut parts = request_text.split("\r\n\r\n");

        let header_part = parts.next()?;
        
        let body = parts
          .next()
          .unwrap_or("")
          .trim_matches(char::from(0));

        let first_line = header_part.lines().next()?;
        let mut first_line_parts = first_line.split_whitespace();

        Some(Request
        {
            method: first_line_parts.next()?.to_string(),
            path: first_line_parts.next()?.to_string(),
            version: first_line_parts.next()?.to_string(),
            body: body.to_string(),
        })
    }
}