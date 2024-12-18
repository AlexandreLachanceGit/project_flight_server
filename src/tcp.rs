use crate::message::{Message, MessagePacket};
use crate::{id, thread_pool::ThreadPool};
use std::{
    io::Write,
    net::{SocketAddr, TcpListener, TcpStream},
};

fn handle_client(mut stream: TcpStream) {
    let client_id = id::new_id();
    info!("Client {client_id} connected");
    stream
        .write_all(&MessagePacket::new(Message::Ping).serialize())
        .unwrap();
}

pub fn start(nb_threads: usize, port: u16) -> std::io::Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let listener = TcpListener::bind(addr)?;
    let thread_pool = ThreadPool::new(nb_threads);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => thread_pool.execute(|| {
                handle_client(stream);
            }),
            Err(err) => {
                error!("Connection failed: {err}");
            }
        }
    }
    Ok(())
}
