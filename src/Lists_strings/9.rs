fn main() {
    let v1 = vec![0, 1];
    let v2 = vec![2, 3];
    assert_eq!(vec![0, 1, 2, 3], lists_add(&v1, &v2));
}

fn lists_add<T>(list1: &Vec<T>, list2: &Vec<T>) -> Vec<T>
    where T: Clone
{
    let mut result: Vec<T> = Vec::new();
    for i in list1 {
        result.push((*i).clone());
    }
    for i in list2 {
        result.push((*i).clone());
    }
    result
}
