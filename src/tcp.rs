use crate::{id, thread_pool::ThreadPool};
use std::net::{SocketAddr, TcpListener, TcpStream};

fn handle_client(stream: TcpStream) {
    let client_id = id::new_id();
    println!("Hello Client: {client_id}");
}

pub fn start(nb_threads: usize, port: u16) -> std::io::Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let listener = TcpListener::bind(addr)?;
    let thread_pool = ThreadPool::new(nb_threads);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Client connected");
                thread_pool.execute(|| {
                    handle_client(stream);
                })
            }
            Err(err) => {
                println!("Connection failed: {err}");
            }
        }
    }
    Ok(())
}
