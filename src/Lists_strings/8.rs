use std::ops::Mul;
use std::fmt::Display;
fn main() {
    let v1 = vec![2; 30];
    let result = on_all(&v1);
}

fn on_all<T>(list: &Vec<T>) -> Vec<T>
    where T: Mul<T, Output = T> + Display + Clone
{
    let mut result = Vec::new();
    for i in list {
        result.push((*i).clone() * (*i).clone());
    }
    if result.len() < 20 {
        panic!("List is too short!");
    } else {
        let mut count = 0;
        for i in &result {
            println!("{}", i);
            count += 1;
            if count == 20 {
                break;
            }
        }
        result
    }
}
