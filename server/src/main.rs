extern crate core;

mod dedicated_server;

fn main() {
    let result = dedicated_server::start();
    match result {
        Ok(_) => {
            println!("Server stopped!");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
