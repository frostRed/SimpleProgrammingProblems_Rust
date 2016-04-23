#![feature(zero_one)]
use std::ops::AddAssign;
use std::num::Zero;
fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = Vec::new();
    assert_eq!(6, list_sum(&v1));
    assert_eq!(0, list_sum(&v2));
}

fn list_sum<T>(list: &Vec<T>) -> T
    where T: AddAssign + Clone + Zero
{
    let mut sum: T = T::zero();
    for i in list {
        sum += (*i).clone();
    }
    sum
}
