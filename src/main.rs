use interprocess::local_socket::{LocalSocketListener, LocalSocketStream};
use ::std::io::{self, prelude::*, BufReader};
use std::time::{Duration};
use std::thread;
use std::fs;
fn handle_error(conn: io::Result<LocalSocketStream>) -> Option<LocalSocketStream> {
    match conn {
        Ok(val) => Some(val),
        Err(error) => {
            eprintln!("Incoming connection failed: {}", error);
            None
        }
    }
}
fn main() {
    thread::spawn(|| {
        let listener = LocalSocketListener::bind("/tmp/test").unwrap();
        for mut conn in listener.incoming().filter_map(handle_error) {
            conn.write_all(b"Hello from server");
            let mut conn2 = BufReader::new(conn);
            let mut buffer = String::new();
            conn2.read_line(&mut buffer);
            println!("Client answered: {}", buffer);
        }
    });
    loop {
        thread::sleep(Duration::from_secs(8));
        let mut test = LocalSocketStream::connect("/tmp/test").unwrap();
        let contents = "Test";
        test.write(contents.as_bytes());
        thread::sleep(Duration::from_secs(1));
    }
}