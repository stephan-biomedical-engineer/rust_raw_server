use mini_servidor::server::server::Server;

#[tokio::main]
async fn main() 
{
    let server = Server::new("127.0.0.1:7878");

    server.run().await;
}