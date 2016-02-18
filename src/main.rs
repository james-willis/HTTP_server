use std::fs::File;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{channel, sync_channel, Sender, Receiver};
use std::thread;
use std::io::{Read, Write};
use chrono::*;
extern crate chrono;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let mut log_file = File::create("log.txt").unwrap();
    let (log, receiver) = channel::<String>();

    //spin up logging thread
    thread::spawn(move|| {
        loop {
            let msg = receiver.recv().unwrap();
            log_file.write(&*msg.into_bytes());
            log_file.write("\n".as_bytes());
        }
    });

    //spin up listener threads
    for stream in listener.incoming() {
        let log = log.clone();
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    handle_client(stream, log);
                });
            },
            Err(e) => {log.send("Failed Connection Attempt".to_string());},
        }
    }

    // close the socket server
    drop(listener);
}

fn handle_client(mut stream: TcpStream, rec: Sender<String>) {
    //determines status of given request,
    //serves the appropriate request, and writes to log
    let time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let mut buf = String::new();
    stream.read_to_string(&mut buf);
    if is_valid_request(&buf) {
        //determine if file exists
            //if exists determine access
                //if access (200)
                //else 403
            //else 404
    }
    else {
        stream.write(&"400 Bad Request: Badly Formatted HTTP request".as_bytes());
        // pretty damn sure this is wrong
    }
}

fn is_valid_request(request: &String) -> bool {
    let mut parsed: Vec<&str> = request.split_whitespace().collect();
    if parsed.len() != 3 {return false;}
    if parsed.pop().unwrap() != "HTTP" {return false;}
    parsed.pop();
    if parsed.pop().unwrap() != "GET" {return false;}
    true
}
