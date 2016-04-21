fn main() {
    use std::io::{self, Write};
    print!("Please enter a Positive Integer number: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim().parse::<u32>() {
        Ok(n) => {
            let num: u32 = n + 1;
            let sum: u32 = (1..num)
                               .filter(|&x| x % 3 == 0 || x % 5 == 0)
                               .fold(0, |s, x| s + x);
            println!("The sum of 1 to {} which is multiples of three or five: {}",
                     num - 1,
                     sum);
        }
        Err(e) => println!("Error: {}", e),
    }
}
