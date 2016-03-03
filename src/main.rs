use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{channel, sync_channel, Sender, Receiver};
use std::thread;
use std::io::BufRead;
use std::io::BufReader;
//use std::sync::{Mutex, Arc};
use std::io::{Read, Write};
use chrono::*;
extern crate chrono;
//#[allow(unused_imports)];
//#[allow(unused_must_use)];

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let mut log_file = File::create("log.txt").unwrap();
    let (log, receiver) = channel::<String>();
    let locked_files: Vec<String> = vec!["src/restricted_text.txt".to_string(), "src/restricted_html.html".to_string()];

    //spin up logging thread
    thread::spawn(move|| {
        loop {
            //****** sanitize input or create a log message. add helper func?
            let msg = receiver.recv().unwrap();
            log_file.write(&*msg.into_bytes());
            //log_file.write("\n".as_bytes());
        }
    });

    //spin up listener threads
    //println!("entering loop");
    for stream in listener.incoming() {
        //println!("Inside loop");
        let log = log.clone();
        let locked_files = locked_files.clone();
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    handle_client(stream, log, locked_files);
                });
            },
            Err(e) => {log.send("Failed Connection Attempt\n".to_string());},
        }
    }

    // close the socket server
    drop(listener);
}

fn handle_client(mut stream: TcpStream, rec: Sender<String>, locked_list: Vec<String>) {
    //determines status of given request,
    //serves the appropriate request, and writes to log
    println!("Client Connected");
    let time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let mut buf:String = String::new();
    let mut input_reader = BufReader::new(stream);
    let len:usize = input_reader.read_line(&mut buf).unwrap();
    stream = input_reader.into_inner();
    rec.send(buf.clone());
    let filepath_result = is_valid_request(&buf);
    if filepath_result.is_ok() {
        let mut path_buffer = PathBuf::from(filepath_result.unwrap());
        let mut ext_flag:bool = false;
        match path_buffer.extension() {
            Some(n) => (),
            None => {
                ext_flag = true;
            }
        }
        if ext_flag {
            path_buffer.push("index");
            path_buffer.set_extension("html");
        }
        let mut filepath = Path::new(path_buffer.as_path());
        let mut target = File::open(&filepath);
        match target {
            Ok(mut target_file) => {
                //if access (200)
                if !locked_list.contains(&filepath.to_str().unwrap().to_string()) {
                    println!("200 OK");
                    rec.send("HTTP/1.0 200 OK\n\n".to_string());
                    let mut success = stream.write(&"HTTP/1.0 200 OK\n".as_bytes()).unwrap();
                    stream.write(&"rsm408/jtw441-web-server\n".as_bytes());

                    if filepath.extension().unwrap() == "html" {
                        stream.write(&"Content-type: text/html\n".as_bytes());
                    }
                    else {
                        stream.write(&"Content-type: text/plain\n".as_bytes());
                    }
                    let mut contents = String::new();
                    let file_size = target_file.read_to_string(&mut contents);
                    let mut size_msg = "Content-length: ".to_string();
                    size_msg.push_str(&file_size.unwrap().to_string());
                    size_msg.push_str("\n\n");
                    stream.write(&size_msg.as_bytes());
                    stream.write(&contents.as_bytes());
                    stream.write("\n".as_bytes());
                    rec.send(contents);
                    rec.send("\n\n".to_string());
                }
                //else 403
                else {
                    println!("403 Forbidden");
                    rec.send("HTTP/1.0 403 Forbidden\n\n".to_string());
                    stream.write(&"HTTP/1.0 403 Forbidden\n\n".as_bytes());
                }
            },
            Err(e) => {
                println!("404 not found");
                rec.send("HTTP/1.0 404 Not Found\n\n".to_string());
                stream.write(&"HTTP/1.0 404 Not Found\n\n".as_bytes());
            }
        }
    }
    else {
        println!("400 bad request");
        stream.write(&"HTTP/1.0 400 Bad Request\n\n".as_bytes());
        rec.send("HTTP/1.0 400 Bad Request\n\n".to_string());
    }
    return
}

fn is_valid_request(request: &String) -> Result<&str, (&str)> {
    let mut parsed: Vec<&str> = request.split_whitespace().collect();
    if parsed.len() != 3 {return Err("Incorrect number of tokens");}
    if !parsed.pop().unwrap().starts_with("HTTP") {return Err("Doesn't end with 'HTTP'");}
    let mut ret: &str = parsed.pop().unwrap();
    ret = &ret[1..];
    if parsed.pop().unwrap() != "GET" {return Err("First argument isn't 'GET'");}
    Ok(ret)
}
