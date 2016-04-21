fn main() {
    let mut sum: f64 = 0.;
    for i in 1..100000000 + 1 {
        sum += (-1i32).pow(i + 1) as f64 / (2 * i - 1) as f64;
    }
    println!("The number is: {}", sum);
}
