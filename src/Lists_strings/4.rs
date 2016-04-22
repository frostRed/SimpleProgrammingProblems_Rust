fn main() {
    let v = vec![1, 2, 3, 4];
    assert_eq!(vec![2, 4], list_odd(&v));
}

fn list_odd<T>(list: &Vec<T>) -> Vec<T>
    where T: Clone
{
    let index = (0..list.len()).filter(|&x| x % 2 == 1);
    let mut retu = Vec::<T>::new();
    for i in index {
        retu.push(list[i].clone());
    }
    retu
}
