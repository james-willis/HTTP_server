use std::fs::File;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{channel, sync_channel, Sender, Receiver};
use std::thread;
use std::io::{Read, Write};
use chrono::*;
extern crate chrono;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    //********* I think log_file needs to live in a mutex?
    let mut log_file = File::create("log.txt").unwrap();
    let (log, receiver) = channel::<String>();

    //spin up logging thread
    thread::spawn(move|| {
        loop {
            //****** sanitize input or create a log message. add helper func?
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
    //determine if file exists
    match let filepath = is_valid_request(&buf) {
        Ok(filepath) => {
            let mut target = File::create(&filepath);
            match target {
                Ok(target) => {
                    //if access (200)
                    //else 403
                },
                Err(e) => {
                    log.send("{}:  404 Error: File {} Does Not Exist", time, &filepath)
                }
            }
        }


            //if exists determine access
            //else 404
    }
    else {
        stream.write(&"400 Bad Request: Badly Formatted HTTP request".as_bytes());
        rec.send("400 Bad Request: Badly Formatted HTTP request".to_string())
        // pretty damn sure this is wrong
    }
}

fn is_valid_request(request: &String) -> Result<&str, ()> {
    let mut parsed: Vec<&str> = request.split_whitespace().collect();
    if parsed.len() != 3 {return false;} // ******* Return Err instead
    //*****changed following str= to a contain, to adjust for version indicator. The spec says only newer versions display their version, so I think we can leave out the case where we get HTTP/0.9
    if !parsed.pop().unwrap().starts_with("HTTP/1.") {return false;} // ******* Return Err instead
    let ret: &str = parsed.pop();
    if parsed.pop().unwrap() != "GET" {return false;} // ******* Return Err instead
    ret
}
