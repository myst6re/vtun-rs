use std::net::TcpStream;
use nix::sys::socket::setsockopt;
use nix::sys::socket::sockopt::KeepAlive;
use std::thread;
use std::fs::File;

// TODO: Note: Maybe we can create a trait with read, write with implicit protocol handled

fn tunnel_net<S: Read + Write + AsRawFd>(in_c: S, out_c: S) {
    let buf = &mut [];

    loop {
        match in_c.read(buf) { // TODO: protocol and headers
            Ok(r) => {
                out_c.write_all(buf);
                out_c.flush();
            }
            Err(e) => {

            }
        }
    }
}

fn tunnel_loc<S: Read + Write + AsRawFd>(in_c: S, out_c: S) {
    let buf = &mut [];

    loop {
        match in_c.read(buf) {
            Ok(r) => {
                out_c.write_all(buf); // TODO: protocol and headers
                out_c.flush();
            }
            Err(e) => {

            }
        }
    }
}

fn tunnel_loop<S: Read + Write + AsRawFd>(net: S, loc: S) {
    thread::spawn(move|| {
        tunnel_net(net, loc);
    });

    thread::spawn(move|| {
        tunnel_loc(loc, net);
    });
}

pub fn tunnel(mut stream: TcpStream, udp: bool, timeout: Option<Duration>) {
    // TODO: auth
    // TODO: open local interface
    let loc = File::open("/dev/tun5").unwrap();
    // TODO: configure stream, or open datagram
    if (udp) {

    } else {
        // Not implemented yet by std::net
        setsockopt(stream.as_raw_fd(), KeepAlive, true);
        stream.set_nodelay(true);
        stream.set_read_timeout(timeout);
        stream.set_write_timeout(timeout);
    }
    tunnel_loop(stream, loc);
}
