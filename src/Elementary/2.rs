use std::sync::{Arc, Mutex};
use std::thread;
fn main() {
    use std::io::{self, Write};
    print!("Please input your name: ");
    io::stdout().flush().unwrap();

    let input = Arc::new(Mutex::new(String::new()));
    let input_shared = input.clone();
    let handle = thread::spawn(move || {
        let mut input_shared = input_shared.lock().unwrap();
        match io::stdin().read_line(&mut input_shared) {
            Ok(_) => println!("Hello, {}", *input_shared),
            Err(error) => println!("Error: {}", error),
        }
    });

    handle.join().unwrap();
}
