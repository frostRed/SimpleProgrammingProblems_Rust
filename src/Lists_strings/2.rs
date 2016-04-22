fn main() {
    let mut v1 = vec![1, 2, 3, 4];
    let mut v2 = vec![1, 2, 3, 4, 5];
    let mut v3: Vec<i32> = Vec::new();
    list_reverse(&mut v1);
    list_reverse(&mut v2);
    list_reverse(&mut v3);
    assert_eq!(vec![4, 3, 2, 1], v1);
    assert_eq!(vec![5, 4, 3, 2, 1], v2);
    assert_eq!(Vec::<i32>::new(), v3);
}

fn list_reverse<T>(list: &mut Vec<T>)
    where T: Clone
{
    let len = list.len() as usize;
    let mid = len / 2;
    for i in 0..mid {
        list.swap(i as usize, len - 1 - i);
    }
}
