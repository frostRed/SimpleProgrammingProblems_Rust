fn main() {
    use std::io::{self, Write};
    let limit = 12;
    for i in 1..limit + 1 {
        for j in 1..limit + 1 {
            print!{"{:>2}x{:<2} = {:<3} ", i, j, i * j};
        }
        print!("\n");
        io::stdout().flush().unwrap();
    }
}
