extern crate core;

mod cocoabeans;

fn main() {
    let result = cocoabeans::server::dedicated_server::start();
    match result {
        Ok(_) => {
            println!("Server stopped!");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
