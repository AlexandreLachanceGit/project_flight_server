use crate::tcp;

pub struct Server;

impl Server {
    pub fn start(tcp_threads: usize) -> std::io::Result<()> {
        tcp::start(tcp_threads)?;

        Ok(())
    }
}
