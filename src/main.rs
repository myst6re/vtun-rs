mod tunnel;
mod server;
mod client;

use server::server;

fn main() {
    

    server("localhost:8080");
}
