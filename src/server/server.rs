use std::net::TcpListener;
use crate::server::thread_pool::ThreadPool;
use crate::server::tcp::handle_connection;


pub struct Server 
{
    address: String,
}

impl Server 
{
    pub fn new(address: &str) -> Server 
    {
        Server 
        {
            address: address.to_string(),
        }
    }

    pub fn run(&self) 
    {
        let listener = TcpListener::bind(&self.address)
          .expect("[ERROR] failed to bind address");

        println!("[SUCCESS] Server running at http://{}", self.address);

        let pool = ThreadPool::new(4);

        for stream in listener.incoming() 
        {
            let stream = stream.expect("[ERROR] Failed to connect");

            pool.execute(move || 
            {
                handle_connection(stream);
            });
        }
    }
}