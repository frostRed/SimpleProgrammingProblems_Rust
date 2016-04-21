use std::io::{self, Write};
fn main() {
    print!("Please enter a Positive Integer number: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim().parse::<i32>() {
        Ok(n) => {
            let num = n + 1;
            handle(num);
        }
        Err(e) => println!("Error: {}", e),
    }
}

fn handle(num: i32) {
    print!("Please enter 1 to compute the sum of 1 to {}, or 2 to compute the product: ",
           num - 1);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    match input.trim() {
        "1" => {
            let sum = (1..num).fold(0, |s, x| s + x);
            println!("The sum is {}", sum);
        }
        "2" => {
            let mul = (1..num).fold(1, |s, x| s * x);
            println!("The product is {}", mul);
        }
        _ => println!("Only 1 or 2 is expected"),
    }

}
