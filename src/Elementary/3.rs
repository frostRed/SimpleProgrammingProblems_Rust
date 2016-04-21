fn main() {
    use std::io::{self, Write};
    print!("Please input your name: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let input_trimed = input.trim();
            if input_trimed == "Alice" || input_trimed == "Bob" {
                println!("Hello, {}", input_trimed);
            } else {
                println!("Sorry, you are not the VIPs");
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}
