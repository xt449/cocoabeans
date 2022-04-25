use std::io;

mod cocoabeans;

fn main() -> io::Result<()> {
    return cocoabeans::server::dedicated_server::start();
}
