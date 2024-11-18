mod server;
mod generator;
use clap::{self, Parser};
use std::env;

#[derive(Parser, Default, Debug)]
struct Args {
    #[clap(short, long, default_value_t = 8080)]
    port: u16,
    #[clap(short, long, default_value_t = String::from("0.0.0.0"))]
    bind_address: String,
}

fn main() {
    std::env::set_var("RUST_LOG", "ntex=info,diesel=debug");
    env_logger::init();
    let mut args = Args::parse();
    match env::var("CALCR_BIND_ADDRESS"){
        Ok(e) => args.bind_address = e,
        _=>()
    }
    match env::var("CALCR_PORT"){
        Ok(e) => args.port = e.parse::<u16>().unwrap(),
        _=>()
    }
    let _ = server::start_server(args);
}