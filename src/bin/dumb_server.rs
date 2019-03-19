#[macro_use] extern crate clap;
use std::net::{SocketAddr, UdpSocket};

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

fn start_tcp_server<S: AsRef<str>>(address_str: S) {}