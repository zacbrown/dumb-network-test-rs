#[macro_use] extern crate clap;
use std::net::{SocketAddr, TcpListener, UdpSocket};
use std::io::Read;

arg_enum! {
    #[derive(Clone, Debug)]
    pub enum Protocol {
        Udp,
        Tcp,
    }
}

fn main() {
    let matches = clap::App::new("dumb_server")
        .version("0.1.0")
        .arg(
            clap::Arg::with_name("protocol")
                .long("protocol")
                .short("p")
                .value_name("PROTOCOL")
                .required(true)
                .case_insensitive(true)
                .possible_values(&Protocol::variants()),
        )
        .arg(
            clap::Arg::with_name("address")
                .long("address")
                .short("a")
                .value_name("SOCKET-ADDRESS")
                .required(true)
                .case_insensitive(true),
        )
        .get_matches();

    let protocol = value_t!(matches.value_of("protocol"), Protocol).unwrap_or_else(|e| e.exit());
    let address_str = matches.value_of("address").unwrap();

    println!("Current process id: {}", std::process::id());

    match protocol {
        Protocol::Udp => start_udp_server(address_str),
        Protocol::Tcp => start_tcp_server(address_str),
    }
}

fn start_udp_server(address_str: &str) {
    let address: SocketAddr = address_str.parse().expect("invalid socket address");
    let socket = UdpSocket::bind(address).expect("couldn't bind to address");

    println!("Starting UDP server listening on {} ...", address);

    let mut buf = [0; 4096];
    let mut count_messages = 0usize;
    loop {
        let (number_of_bytes, src_addr) = socket.recv_from(&mut buf)
            .expect("Didn't receive data");
        count_messages += 1;

        if count_messages % 1000 == 0 {
            println!("{} messages processed. Current: src_addr={},num_bytes={}", count_messages, src_addr, number_of_bytes);
        }
    }
}

fn start_tcp_server(address_str: &str) {
    let address: SocketAddr = address_str.parse().expect("invalid socket address");
    let listener = TcpListener::bind(address).expect("couldn't bind to address");
    let mut buf = vec![0u8; 16];
    let mut count_messages = 0usize;

    println!("Starting TCP server listening on {} ...", address);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let src_addr = stream.peer_addr().expect("couldn't get address of source");
                let number_of_bytes = stream.read_to_end(&mut buf).expect("unable to read from stream");
                count_messages += 1;

                println!("Incoming message read...");
                if count_messages % 1000 == 0 {
                    println!("{} messages processed. Current: src_addr={},num_bytes={}", count_messages, src_addr, number_of_bytes);

                }
            }
            Err(_) => {}
        }
    }
}