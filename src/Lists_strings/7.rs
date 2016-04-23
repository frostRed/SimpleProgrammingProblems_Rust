#![feature(zero_one)]
use std::ops::AddAssign;
use std::ops::Add;
use std::num::Zero;

fn main() {
    let v1 = vec![1, 2, 3];
    assert_eq!(6, list_sum_recu(&v1[..]));
}

fn list_sum_for<T>(list: &Vec<T>) -> Option<T>
    where T: AddAssign + Clone + Zero
{
    if list.is_empty() {
        return None;
    }
    let mut sum = T::zero();
    for i in list {
        sum += (*i).clone()
    }
    Some(sum)
}
fn list_sum_while<T>(list: &Vec<T>) -> Option<T>
    where T: AddAssign + Clone + Zero
{
    if list.is_empty() {
        return None;
    }
    let mut sum = T::zero();

    let mut index: usize = list.len() - 1;
    while index >= 0 {
        sum += list[index].clone();
        index -= 1;
    }
    Some(sum)
}
fn list_sum_recu<T>(list: &[T]) -> T
    where T: Add
{
    let mut sum = list[0] + list_sum_recu(&list[1..]);
    sum
}
