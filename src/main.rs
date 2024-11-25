mod id;
mod server;
mod tcp;
mod thread_pool;

use clap::Parser;

use server::Server;

/// Server for Project Flight
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of threads for TCP server
    #[arg(short, long, default_value_t = 4)]
    tcp_threads: usize,

    /// Port for TCP server
    #[arg(short = 'p', long, default_value_t = 4001)]
    tcp_port: u16,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    Server::start(args.tcp_threads, args.tcp_port)
}
