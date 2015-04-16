extern crate clock_ticks;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::Result as IoResult;
use std::io::Error as IoError;
use std::io::ErrorKind;
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket, IpAddr, Ipv4Addr};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread::spawn;

fn time_ms() -> u64 {
    use clock_ticks;
    clock_ticks::precise_time_ns() / 1000
}

struct FakeUdp {
    pub latency: u32,
    pub latency_modulation: f32,
    pub drop_rate: f32,

    send: Sender<Vec<u8>>,
    recv: Receiver<Vec<u8>>
}

trait MockUdp {
    fn recv_from(&self, buf: &mut [u8]) -> IoResult<(usize, SocketAddr)>;
    fn send_to<A: ToSocketAddrs>(&self, buf: &[u8], addr: A) -> IoResult<usize>;
}

impl MockUdp for UdpSocket {
    fn recv_from(&self, buf: &mut [u8]) -> IoResult<(usize, SocketAddr)> {
        self.recv_from(buf)
    }
    fn send_to<A: ToSocketAddrs>(&self, buf: &[u8], addr: A) -> IoResult<usize> {
        self.send_to(buf, addr)
    }
}

#[derive(Eq, PartialEq)]
struct PacketAndTime {
    time: u64,
    packet: Vec<u8>
}

impl Ord for PacketAndTime {
    fn cmp(&self, other: &PacketAndTime) -> Ordering {
        self.time.cmp(&other.time)
    }
}

impl PartialOrd for PacketAndTime {
    fn partial_cmp(&self, other: &PacketAndTime) -> Option<Ordering> {
        self.time.partial_cmp(&other.time)
    }
}

impl FakeUdp {
    pub fn pair() -> (FakeUdp, FakeUdp) {
        let (a_send_s, a_send_r) = channel();
        let (a_recv_s, a_recv_r) = channel();

        let (b_send_s, b_send_r) = channel();
        let (b_recv_s, b_recv_r) = channel();

        spawn(move || {
            let a_send_r = a_send_r;
            let a_recv_s = a_recv_s;

            let b_send_r = b_send_r;
            let b_recv_s = b_recv_s;

            let mut a_queue = BinaryHeap::new();
            let mut b_queue = BinaryHeap::new();

            loop {
                let now = time_ms();

                if let Some(head_a) = a_queue.peek() {

                }
            }
        });

        let a = FakeUdp {
            latency: 0,
            latency_modulation: 0.0,
            drop_rate: 0.0,
            send: a_send_s,
            recv: a_recv_r,
        };

        let b = FakeUdp {
            send: b_send_s,
            recv: b_recv_r,
            ..a
        };

        (a, b)
    }
}

impl MockUdp for FakeUdp {
    fn recv_from(&self, buf: &mut [u8]) -> IoResult<(usize, SocketAddr)> {
        match self.recv.recv() {
            Ok(v) => {
                let size = std::cmp::min(buf.len(),v.len());
                for i in 0 .. size {
                    buf[i] = v[i];
                }
                Ok((size, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0,0,0,0)), 0)))
            }
            Err(_) => {
                Err(IoError::new(ErrorKind::ConnectionAborted, "The other end closed."))
            }
        }
    }

    fn send_to<A: ToSocketAddrs>(&self, buf: &[u8], addr: A) -> IoResult<usize> {

    }
}
