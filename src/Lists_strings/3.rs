fn main() {
    let v1 = vec![1, 2, 3];
    assert_eq!(Some(true), list_check(&v1, 3));
    assert_eq!(Some(false), list_check(&v1, 4));
    assert_eq!(None, list_check(&Vec::<i32>::new(), 4));
}

fn list_check<T>(list: &Vec<T>, ele: T) -> Option<bool>
    where T: PartialEq
{
    if list.is_empty() {
        return None;
    }
    let mut result = false;
    for i in list {
        if *i == ele {
            result = true;
        }
    }
    Some(result)
}
