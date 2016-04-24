fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];
    let v3 = vec!["a", "b", "c"];
    assert_eq!(vec![(1, 4), (2, 5), (3, 6)], lists_mix(&v1, &v2).unwrap());
    assert_eq!(vec![(1, "a"), (2, "b"), (3, "c")],
               lists_mix(&v1, &v3).unwrap());
}

fn lists_mix<T, H>(list1: &Vec<T>, list2: &Vec<H>) -> Option<Vec<(T, H)>>
    where T: Clone,
          H: Clone
{
    if list1.len() != list2.len() {
        return None;
    }
    let mut result = Vec::new();
    for i in list1.into_iter().zip(list2.into_iter()) {
        let (ele1, ele2) = ((*i.0).clone(), (*i.1).clone());
        result.push((ele1, ele2));
    }
    Some(result)

}
