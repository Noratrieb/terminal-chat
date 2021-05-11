use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::error::Error;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

type StreamResult = Result<TcpStream, Box<dyn Error>>;


pub fn listen() -> StreamResult {
    println!("Listening on {}:8080", local_ipaddress::get().unwrap());
    let listener = TcpListener::bind(("127.0.0.1", 8080))?;
    let stream = listener.incoming().next().unwrap()?;
    println!("Connected. Waiting for response stream...");
    println!("other adress: {}", stream.peer_addr()?.ip());

    Ok(stream)
}

pub fn connect(address: String, port: String) -> StreamResult {
    println!("Trying to connect to '{}:{}'", &address, &port);

    let port: u16 = port.parse()?;
    let stream = TcpStream::connect((&*address, port))?;

    println!("Connected.");

    Ok(stream)
}


pub fn network_thread(mut stream: TcpStream, rx: Receiver<String>) {
    let mut data = [0u8; 255]; // using 50 byte buffer
    stream.set_read_timeout(Some(Duration::from_millis(500))).expect("Could not set timeout");
    loop {
        if let Ok(_) = stream.read(&mut data) {
            println!("> {}", String::from_utf8_lossy(&data));
            data = [0u8; 255];
        }
        if let Ok(v) = rx.recv_timeout(Duration::from_millis(200)) {
            stream.write(v.as_bytes()).expect("could not send message");
        }
    }
}

pub fn ui_thread(sx: Sender<String>) {
    loop {
        let input = input();
        sx.send(input).expect("could not send value");
    }
}


pub fn input() -> String {
    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);
    buffer.trim().to_string()
}
