use std::net::{TcpStream, TcpListener, IpAddr, Ipv4Addr};
use std::io::{self, Read};
use std::sync::{Mutex, Arc};
use crate::IpAddress::{Normal, WithPort};

pub enum IpAddress {
    WithPort(String, u32),
    Normal(String)
}

pub fn connect(address: IpAddress) -> io::Result<TcpStream> {
    let address = match address {
        Normal(s) => {
            match &*s {
                "default" => String::from("localhost:7979"),
                _ => s
            }
        },
        WithPort(a, p) => format!("{}:{}", a, p)
    };

    TcpStream::connect(address)
}

pub fn listen(port: usize) -> io::Result<TcpStream> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    println!("Listening on: {}:{}...", local_ipaddress::get().unwrap(), 7979);

    listener.incoming().next().unwrap()
}

pub fn read_stream_print(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer

    for _ in stream.read(&mut data) {
        println!("{}", String::from_utf8_lossy(&data));
    }
}