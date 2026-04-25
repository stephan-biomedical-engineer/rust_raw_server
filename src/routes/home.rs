use crate::http::response::Response;

pub fn home() -> Response 
{
    Response::ok("<h1>Home</h1><p>Servidor Rust funcionando.</p>")
}

pub fn about() -> Response 
{
    Response::ok("<h1>Sobre</h1><p>Mini backend em Rust sem framework.</p>")
}