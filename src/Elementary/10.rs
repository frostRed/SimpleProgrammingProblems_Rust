fn main() {
    use std::io::{self, Write};
    let leap = (2016..)
                   .filter(|&x| x % 400 == 0 || (x % 4 == 0 && x % 100 != 0))
                   .take(20);
    print!("Next 20 leaps is:");
    for i in leap {
        print!(" {}", i)
    }
    print!("\n");
    io::stdout().flush().unwrap();
}
