use std::fs::File;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
extern crate chrono;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let log = File::create("log.txt").unwrap();
    let log_mut = Arc::new(Mutex::new(log));
    for stream in listener.incoming() {
        let log_mut = log_mut.clone();
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    // connection succeeded
                    unimplemented!();
                    handle_client(stream);
                });
            },
            Err(e) => write_log(log_mut, "Failed Connection"),
        }
    }

    // close the socket server
    drop(listener);
}

fn handle_client(stream: TcpStream) {
    //determines status of given request,
    //spins up a new thread and writes to
    //the log
    unimplemented!();
}

fn write_log<S: Into<String>>(log: Arc<Mutex<File>>, entry: S) {
    // prints timestamp and request info to the log
    // acquire a lock on the file, and writes S
    let entry = entry.into();
    unimplemented!();
    //I added a crate for timestamping, docs here:
    //https://lifthrasiir.github.io/rust-chrono/chrono/
}
