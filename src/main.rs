use std::thread;
use terminal_chat::{connect, listen, network_thread, input, ui_thread};
use std::sync::{mpsc};
use std::sync::mpsc::{Sender, Receiver};

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

    println!("Do you want to (l)isten or (c)onnect to a listener?");

    let result = if input() == "l" {
        listen()
    } else {
        println!("Address: (empty for default)");
        let address = match &*input() {
            "" => String::from("127.0.0.1"),
            s => String::from(s)
        };
        println!("Port: (empty for default)");
        let port = match &*input() {
            "" => String::from("8080"),
            s => String::from(s)
        };
        connect(address, port)
    };

    match result {
        Ok(s) => {
            let (sx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

            let net = thread::spawn(move || {
                network_thread(s, rx);
            });
            let ui = thread::spawn(move ||{
                ui_thread(sx)
            });

            net.join().unwrap();
            ui.join().unwrap();
        }
        Err(e) => println!("An error occurred! Error message: '{}'", e)
    }
}