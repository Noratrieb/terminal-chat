use std::io::{self, Write, Read};
use terminal_chat::{connect, listen, read_stream_print, IpAddress};
use std::thread;
use std::sync::{Arc, Mutex};


fn main() {
    println!(r"
 _______                  _             _  _____ _           _
|__   __|                (_)           | |/ ____| |         | |
   | | ___ _ __ _ __ ___  _ _ __   __ _| | |    | |__   __ _| |_
   | |/ _ \ '__| '_ ` _ \| | '_ \ / _` | | |    | '_ \ / _` | __|
   | |  __/ |  | | | | | | | | | | (_| | | |____| | | | (_| | |_
   |_|\___|_|  |_| |_| |_|_|_| |_|\__,_|_|\_____|_| |_|\__,_|\__|
---------------------------------------------------------------
Version 0.1");

    println!("Do you want to listen(l) or connect(c) to a listener?");
    let mut did_listen;
    let stream =
        if input().contains("l") {
            did_listen = true;
            listen(7979)
        } else {
            did_listen = false;
            let mut address = input().trim().to_string();
            println!("Trying to connect to '{}'", address.clone());

            connect(IpAddress::Normal(address.clone()))
        };

    let stream = match stream {
        Ok(x) => x,
        Err(_) => {
            println!("Error opening stream");
            return;
        }
    };

    let mut stream2 = match did_listen {
        true => connect(IpAddress::WithPort(String::from("127.0.0.1"), 7980)).unwrap(),
        false => listen(7989).unwrap()
    };

    thread::spawn(move || {
        read_stream_print(stream);
    });

    println!("Connected.");

    let connect_msg = "Hello World!";
    stream2.write(connect_msg.as_bytes()).unwrap();

    loop {
        let input = input();
        stream2.write(input.clone().as_bytes());
        println!("-- Trying to send {}", input);
    }
}


fn input() -> String {
    print!("> ");
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);
    buffer
}