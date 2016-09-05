use std::net::{TcpStream, ToSocketAddrs};
use tunnel::tunnel;

pub fn client<A: ToSocketAddrs>(target_addr: A) {
    // TODO: connect timeout
    // TODO: bind to local address if needed to choose the iface
    let mut stream = TcpStream::connect(target_addr).unwrap();
    tunnel(stream);
}
