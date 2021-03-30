use std::io::{self, Read};
use std::net::{TcpListener, TcpStream, IpAddr, SocketAddr, Ipv4Addr, SocketAddrV4};
use std::error::Error;
use std::str::FromStr;

type EmptyResult = Result<(), Box<dyn Error>>;


pub fn listen() -> EmptyResult {
    let l_stream = listen_on_port(7979)?;
    println!("Connected. Waiting for response stream...");
    let other_address = l_stream.peer_addr()?.ip();
    println!("other adress: {}", other_address);
    let s_stream = connect_to_addr("127.0.0.1", 7980)?;


    Ok(())
}

pub fn connect(address: String, port: String) -> EmptyResult {
    println!("Trying to connect to '{}:{}'", &address, &port);

    let port: u16 = port.parse()?;
    let s_stream = connect_to_addr(&*address, port)?;

    println!("Connected. Waiting for response stream...");
    let l_stream = listen_on_port(7080)?;
    println!("Connected. Chat is now open...");

    Ok(())
}

pub fn connect_to_addr(address: &str, port: u16) -> Result<TcpStream, Box<dyn Error>> {
    let socket_addr = SocketAddrV4::new(Ipv4Addr::from_str(address)?, port);
    match TcpStream::connect(socket_addr) {
        Err(e) => Err(Box::new(e)),
        Ok(s) => Ok(s)
    }
}

pub fn listen_on_port(port: u16) -> io::Result<TcpStream> {
    let listener = TcpListener::bind(("127.0.0.1", port)).unwrap();
    println!(
        "Listening on: {}:{}...",
        local_ipaddress::get().unwrap(),
        port
    );

    listener.incoming().next().unwrap()
}

pub fn read_stream_print(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer

    for _ in stream.read(&mut data) {
        println!("{}", String::from_utf8_lossy(&data));
    }
}


fn input() -> String {
    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);
    buffer
}
