use std::io;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9090").unwrap();
    listener.set_nonblocking(true).expect("Cannot set non-blocking");
    
    loop {
        let num_events = epoll_wait(
            epollfd, pevents, NUMEVENTS, 10000
        ).unwrap();

        for(
    }
}
