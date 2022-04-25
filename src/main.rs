mod cocoabeans;

use std::io;

fn main() -> io::Result<()> {
    return cocoabeans::server::dedicated_server::start();
}
