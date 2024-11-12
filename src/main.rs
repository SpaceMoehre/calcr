use ntex::web;
mod generator;
mod server;

fn main() {
    match server::start_server(){
        Err(e) => panic!("Could not start Server {e:?}"),
        _ => println!("Started Server on 0.0.0.0:28080")
    }
}
    