mod thread_pool;

use std::net::{TcpListener, TcpStream};

use thread_pool::ThreadPool;

fn handle_client(stream: TcpStream) {
    println!("Hello");
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:5000")?;
    let thread_pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            println!("Client connected");
            thread_pool.execute(|| {
                handle_client(stream);
            })
        } else {
            println!("Connection failed");
        }
    }
    Ok(())
}

