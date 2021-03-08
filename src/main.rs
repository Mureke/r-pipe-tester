use ::std::io::{self, prelude::*, BufReader};
use std::time::{Duration};
use std::thread;
use std::fs;

use interprocess::local_socket::{LocalSocketListener, LocalSocketStream};
use shutdown_hooks::add_shutdown_hook;
fn handle_error(conn: io::Result<LocalSocketStream>) -> Option<LocalSocketStream> {
    match conn {
        Ok(val) => Some(val),
        Err(error) => {
            eprintln!("Incoming connection failed: {}", error);
            None
        }
    }
}


extern "C" fn shutdown_hook(){
    fs::remove_file("/tmp/test");
}


fn main() {
    add_shutdown_hook(shutdown_hook);

    // Pipe server
    thread::spawn(|| {
        let listener = LocalSocketListener::bind("/tmp/test").unwrap();
        for mut conn in listener.incoming().filter_map(handle_error) {
            let mut contents = "Server response";
            conn.write_all(contents.as_bytes());

            let mut buffer = String::new();
            conn.read_to_string(&mut buffer);
            println!("Client request: {}", buffer);

        }
    });
    thread::sleep(Duration::from_secs(3));

    let mut i: u32 = 0;
    loop {
        // WRITE
        let mut stream = LocalSocketStream::connect("/tmp/test").unwrap();
        stream.set_nonblocking(true);
        let mut contents = i.to_string();
        stream.write_all(contents.as_bytes());
        i = i+1;

        thread::sleep(Duration::from_secs(8));

        // Read from pipe
        let mut buffer = String::new();
        stream.read_to_string(&mut buffer);
        println!("Server answered: {}", buffer);
    }
}