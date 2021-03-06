use ::std::io::{self, prelude::*, BufReader};
use std::time::{Duration};
use std::thread;
use std::fs;
use std::env;

use interprocess::local_socket::{LocalSocketListener, LocalSocketStream};
use shutdown_hooks::add_shutdown_hook;
use std::env::args;

const PIPE_NAME: &str = "/tmp/test";

fn handle_error(conn: io::Result<LocalSocketStream>) -> Option<LocalSocketStream> {
    match conn {
        Ok(val) => Some(val),
        Err(error) => {
            eprintln!("Incoming connection failed: {}", error);
            None
        }
    }
}


extern "C" fn shutdown_hook() {
    fs::remove_file(PIPE_NAME);
}


fn main() {
    add_shutdown_hook(shutdown_hook);

    let args: Vec<String> = env::args().collect();
    let mut is_testing = "false";
    if args.len() > 1 {
        is_testing = &args[1];
    }

    println!("TESTING: {}", is_testing);
    if is_testing == "true" {
        let _write_thread = thread::spawn(|| {
            thread::sleep(Duration::from_secs(3));

            let mut i: u32 = 0;
            loop {
                // WRITE
                let mut stream = LocalSocketStream::connect(PIPE_NAME).unwrap();
                stream.set_nonblocking(true);
                let mut contents = i.to_string();
                stream.write_all(contents.as_bytes());
                i = i + 1;
                thread::sleep(Duration::from_secs(3));

                // READ
                let mut buffer = String::new();
                stream.read_to_string(&mut buffer);
                println!("Server answered: {}", buffer);
                thread::sleep(Duration::from_secs(3));
            }
        });
    }

    let listener = LocalSocketListener::bind(PIPE_NAME).unwrap();
    for mut conn in listener.incoming().filter_map(handle_error) {
        conn.set_nonblocking(true);

        let mut buffer = String::new();
        conn.read_to_string(&mut buffer);
        println!("Client request: {}", buffer);

        let mut contents = format!("ẞerver Response {}", buffer);
        conn.write_all(contents.as_bytes());
    }
}