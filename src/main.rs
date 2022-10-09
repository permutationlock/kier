use std::io;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::collections::VecDeque;
use std::os::unix::io::{AsRawFd, RawFd};

#[allow(unused_macros)]
macro_rules! syscall {
    ($fn: ident ( $($arg: expr),* $(,)* ) ) => {{
        match unsafe { libc::$fn($($arg, )*) } {
            -1 => Err(io::Error::last_os_error()),
            res => Ok(res)
        }
    }}
}

const BUFFSIZE: usize = 50;
const NCONNS: usize = 100;
const LISTEN_IDX: u64 = !(0u64);

pub struct ConnectionContext {
    stream: TcpStream,
    pub buffer: [u8; BUFFSIZE],
}

impl ConnectionContext {
    fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            buffer: [0; BUFFSIZE],
        }
    }

    fn read(&mut self) -> io::Result<usize> {
        self.buffer = [0; BUFFSIZE];
        self.stream.read(&mut self.buffer)
    }
}

pub struct ConnectionSet<const N: usize> {
    conns: [Option<ConnectionContext>; N],
    free_conns: VecDeque<usize>,
}

impl<const N: usize> ConnectionSet<N> {
    fn new() -> Self {
        const NONECONTEXT: Option<ConnectionContext> = None;
        Self {
            conns: [NONECONTEXT; N],
            free_conns: VecDeque::from(
               (0..(N-1)).collect::<Vec<usize>>()
            ),
        }
    }

    fn add(
        &mut self, conn_ctx: ConnectionContext, epoll_fd: RawFd
    ) -> Option<usize> {
        match self.free_conns.pop_front() {
            None => None,
            Some(i) => {
                let fd = conn_ctx.stream.as_raw_fd();
                self.conns[i] = Some(conn_ctx);
                add_event(
                    epoll_fd,
                    fd,
                    get_revent(i as u64)
                ).expect("error adding event");
                Some(i)
            },
        }
    }

    fn remove(&mut self, i: usize, epoll_fd: RawFd) {
        if let Some(conn) = &self.conns[i] {
            let fd = conn.stream.as_raw_fd();
            conn.stream.shutdown(std::net::Shutdown::Both).unwrap();
            remove_event(epoll_fd, fd).unwrap();
            let _ = syscall!(close(fd));
        }
        self.conns[i] = None;
        self.free_conns.push_back(i);
    }

    fn read(&mut self, i: usize) -> io::Result<usize> {
        match &mut self.conns[i] {
            Some(conn) => conn.read(),
            None => Err(std::io::Error::new(
                std::io::ErrorKind::Other, 
                "can't read empyconnection"
            )),
        }
    }
}

fn get_revent(val: u64) -> libc::epoll_event {
    libc::epoll_event {
        events: libc::EPOLLIN as u32,
        u64: val,
    }
}

/*fn get_rwevent(val: u64) -> libc::epoll_event {
    libc::epoll_event {
        events: (libc::EPOLLIN | libc::EPOLLOUT) as u32,
        u64: val,
    }
}*/

fn add_event(
    epoll_fd: RawFd, fd: RawFd, mut event: libc::epoll_event
) -> io::Result<()> {
    syscall!(epoll_ctl(
        epoll_fd, libc::EPOLL_CTL_ADD, fd, &mut event
    ))?;
    Ok(())
}

fn remove_event(
    epoll_fd: RawFd, fd: RawFd
) -> io::Result<()> {
    syscall!(epoll_ctl(
        epoll_fd, libc::EPOLL_CTL_DEL, fd, std::ptr::null_mut()
    ))?;
    Ok(())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9090").unwrap();
    listener.set_nonblocking(true).expect("Cannot set non-blocking");

    let mut cset = ConnectionSet::<NCONNS>::new();

    let mut events: Vec<libc::epoll_event>
        = Vec::with_capacity(NCONNS + 1);

    let epoll_fd = syscall!(epoll_create(0xf00d))
        .expect("failed to initialize epoll fd");

    add_event(
        epoll_fd, listener.as_raw_fd(), get_revent(LISTEN_IDX)
    ).expect("failed to register listener");

    loop {
        events.clear();
        let res = match syscall!(epoll_wait(
            epoll_fd,
            events.as_mut_ptr() as *mut libc::epoll_event,
            (NCONNS + 1) as i32,
            1000 as libc::c_int
        )) {
            Ok(n) => n,
            Err(e) => panic!("error during wait: {}", e),
        };

        unsafe { events.set_len(res as usize) };

        for ev in &events {
            match ev.u64 {
                LISTEN_IDX => {
                    match listener.accept() {
                        Ok((stream, addr)) => {
                            stream.set_nonblocking(true)?;
                            println!("client connected: {}", addr);
                            let _ = cset.add(
                                ConnectionContext::new(stream),
                                epoll_fd
                            ).unwrap();
                        },
                        Err(e) => eprintln!("accept failed: {}", e),
                    }
                },
                i => {
                    let index = i as usize;
                    match cset.read(index) {
                        Ok(0) => {
                            println!("connection {} closed", index);
                            cset.remove(index, epoll_fd);
                        },
                        Ok(n) => {
                            if let Some(conn) = &cset.conns[index] {
                                let msg = std::str::from_utf8(
                                    &conn.buffer[0..n]
                                ).unwrap();
                                println!(
                                    "connection {} sent: {}",
                                    index,
                                    msg
                                );
                            }
                        },
                        Err(e) => println!("error reading: {}", e),
                    }
                },
            }
        }
    }
}
