fn main() {
    let v = vec![1, 2, 5, 3];
    assert_eq!(Some(5), list_max(&v));
    assert_eq!(None, list_max(&Vec::<i32>::new()));
}

fn list_max<T>(list: &Vec<T>) -> Option<T>
    where T: PartialOrd + Clone
{
    if list.is_empty() {
        return None;
    }
    let mut max: T = list[0].clone();
    for i in list {
        if *i > max {
            max = i.clone();
        }
    }
    Some(max)
}
