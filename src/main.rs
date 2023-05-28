use std::io;
use std::io::Write;
use std::net::{IpAddr, Ipv6Addr, SocketAddr, UdpSocket};
use std::thread;

fn main() -> io::Result<()> {
    let addr = SocketAddr::new(Ipv6Addr::UNSPECIFIED.into(), 12345);

    eprintln!("binding to {}", addr);
    let socket = UdpSocket::bind(addr)?;

    thread::scope(|s| -> io::Result<()> {
        for tnr in 1..=2 {
            let socket = socket.try_clone()?;

            s.spawn(move || -> io::Result<()> {
                let mut recvbuf = [0; 128];
                let mut outbuf = Vec::<u8>::with_capacity(128);
                let (mut handled, mut tread, mut twrote) = (0, 0, 0);

                loop {
                    let (read_bytes, from) = match socket.recv_from(&mut recvbuf) {
                        Ok(x) => x,
                        Err(e) => {
                            eprintln!("thread {}: recv_from failed: {}", tnr, e);
                            continue;
                        }
                    };

                    tread += read_bytes;

                    outbuf.clear();

                    if let Err(e) = writeln!(
                        &mut outbuf,
                        "{}",
                        match from.ip() {
                            v4 @ IpAddr::V4(_) => v4,
                            v6 @ IpAddr::V6(x) =>
                                x.to_ipv4_mapped().map(|x| x.into()).unwrap_or(v6),
                        }
                    ) {
                        eprintln!("thread {}: buffer write failed: {}", tnr, e);
                        continue;
                    }

                    // don't send more bytes than we were sent, to avoid being a vector in an
                    // amplification attack.
                    outbuf.truncate(read_bytes);

                    match socket.send_to(&outbuf, from) {
                        Ok(_) => twrote += outbuf.len(),
                        Err(e) => eprintln!("thread {}: write failed to {}: {}", tnr, from, e),
                    };

                    handled += 1;
                    if handled % 1_000 == 0 {
                        eprintln!(
                            "thread {}: handled {} messages (read {} wrote {})",
                            tnr, handled, tread, twrote
                        );
                    }
                }
            });

            eprintln!("started thread {} listening on {}", tnr, addr);
        }

        Ok(())
    })?;

    Ok(())
}
