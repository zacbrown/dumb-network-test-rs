#[macro_use] extern crate clap;
use std::net::{SocketAddr, TcpStream, UdpSocket};
use std::io::Write;

arg_enum! {
    #[derive(Clone, Debug)]
    pub enum Protocol {
        Udp,
        Tcp,
    }
}

fn main() {
    let matches = clap::App::new("dumb_client")
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
            clap::Arg::with_name("server-address")
                .long("server-address")
                .short("s")
                .value_name("SERVER-ADDRESS")
                .required(true)
                .case_insensitive(true),
        )
        .arg(
            clap::Arg::with_name("client-address")
                .long("client-address")
                .short("c")
                .value_name("CLIENT-ADDRESS")
                .required(true)
                .case_insensitive(true),
        )
        .get_matches();

    let protocol = value_t!(matches.value_of("protocol"), Protocol).unwrap_or_else(|e| e.exit());
    let client_address_str = matches.value_of("client-address").unwrap();
    let server_address_str = matches.value_of("server-address").unwrap();

    println!("Current process id: {}", std::process::id());

    match protocol {
        Protocol::Udp => start_udp_client(client_address_str, server_address_str),
        Protocol::Tcp => start_tcp_client(client_address_str, server_address_str),
    }
}

fn start_udp_client(client_address_str: &str, server_address_str: &str) {
    let client_address: SocketAddr = client_address_str.parse().expect("invalid socket address");
    let socket = UdpSocket::bind(client_address).expect("couldn't bind to address");
    socket.connect(server_address_str).expect("couldn't connect to server address");

    println!("Starting UDP client with address {}, connected to {} ...", client_address_str, server_address_str);

    let buf = [1; 4096];
    for ii in 1..11 {
        let _ = socket.send(&buf)
            .expect("Couldn't send data");
        println!("sending message {}", ii);
        std::thread::sleep_ms(1000);
    }
}

fn start_tcp_client(_client_address_str: &str, server_address_str: &str) {
    let server_address: SocketAddr = server_address_str.parse().expect("invalid socket address");
    let mut stream = TcpStream::connect(server_address).expect("couldn't bind to address");

    println!("Starting TCP client with address {}, connected to {} ...",
             stream.local_addr().expect("couldn't get local address"),
             server_address_str);

    let buf = [1; 16];
    for ii in 1..11 {
        let _ = stream.write_all(&buf)
            .expect("couldn't send to remote address");
        println!("sending message {}", ii);
        std::thread::sleep_ms(1000);
    }
}