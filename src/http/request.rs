#[derive(Debug)]
pub struct Request 
{
    pub method: String,
    pub path: String, 
    pub version: String,
}

impl Request
{
    pub fn from_buffer(buffer: &[u8]) -> Option<Request>
    {
        let request_text = String::from_utf8_lossy(buffer);
        let first_line = request_text.lines().next()?;
        let mut parts = first_line.split_whitespace();

        Some(Request
        {
            method: parts.next()?.to_string(),
            path: parts.next()?.to_string(),
            version: parts.next()?.to_string(),
        })
    }
}