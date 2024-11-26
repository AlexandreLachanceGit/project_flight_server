#[macro_use]
extern crate log;
extern crate simplelog;

mod id;
mod server;
mod tcp;
mod thread_pool;

use clap::Parser;
use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode};

use server::Server;

/// Server for Project Flight
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of threads for TCP server
    #[arg(short, long, default_value_t = 4)]
    tcp_threads: usize,

    /// Port for TCP server
    #[arg(short = 'p', long, default_value_t = 5000)]
    tcp_port: u16,
}

fn main() -> std::io::Result<()> {
    TermLogger::init(
        LevelFilter::Trace,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();

    let args = Args::parse();

    Server::start(args.tcp_threads, args.tcp_port)
}
