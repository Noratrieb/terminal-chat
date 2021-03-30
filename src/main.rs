use std::io::{self, Write};
use std::thread;
use terminal_chat::{
    connect, listen, listen_on_port, read_stream_print,
};

fn main() {
    println!(
        r"
 _______                  _             _  _____ _           _
|__   __|                (_)           | |/ ____| |         | |
   | | ___ _ __ _ __ ___  _ _ __   __ _| | |    | |__   __ _| |_
   | |/ _ \ '__| '_ ` _ \| | '_ \ / _` | | |    | '_ \ / _` | __|
   | |  __/ |  | | | | | | | | | | (_| | | |____| | | | (_| | |_
   |_|\___|_|  |_| |_| |_|_|_| |_|\__,_|_|\_____|_| |_|\__,_|\__|
---------------------------------------------------------------
Version 0.1"
    );

    println!("Do you want to listen(l) or connect(c) to a listener?");

    let result = if input().contains("l") {
        listen()
    } else {
        println!("Address: (empty for default)");
        let address = match &*input() {
            "" => String::from("127.0.0.1"),
            s => String::from(s)
        };
        println!("Port: (empty for default)");
        let port = match &*input() {
            "" => String::from("7979"),
            s => String::from(s)
        };
        connect(address, port)
    };

    match result {
        Ok(_) => println!("Exited TerminalChat sucessfully."),
        Err(e) => println!("An error occurred! Error message: '{}'", e)
    }

    // old
    /*

        let stream = match stream {
            Ok(x) => x,
            Err(_) => {
                println!("Error opening stream");
                return;
            }
        };

        let mut stream2 = match did_listen {
            true => connect_to_addr(IpAddress::WithPort(String::from("127.0.0.1"), 7980)).unwrap(),
            false => listen_on_port(7989).unwrap(),
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
        }*/
}

fn input() -> String {
    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);
    buffer
}
