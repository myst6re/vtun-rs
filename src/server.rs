use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::thread;
use tunnel::tunnel;

pub fn server<A: ToSocketAddrs>(addr: A) {
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    tunnel(stream)
                });
            }
            Err(e) => { /* connection failed */ }
        }
    }
}
