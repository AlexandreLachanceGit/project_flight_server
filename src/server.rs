use crate::tcp;

pub struct Server;

impl Server {
    pub fn start(tcp_threads: usize, tcp_port: u16) -> std::io::Result<()> {
        tcp::start(tcp_threads, tcp_port)?;

        Ok(())
    }
}
