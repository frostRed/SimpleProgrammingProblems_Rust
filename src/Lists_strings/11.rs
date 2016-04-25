use std::cmp::PartialOrd;
fn main() {
    let v1 = vec![1, 3, 5];
    let v2 = vec![2, 4];
    assert_eq!(vec![1, 2, 3, 4, 5], list_sorted_merge(&v1, &v2));
    assert_eq!(vec![1, 3, 5], list_sorted_merge(&v1, &Vec::<i32>::new()));
}

fn list_sorted_merge<T>(list1: &Vec<T>, list2: &Vec<T>) -> Vec<T>
    where T: Clone + PartialOrd
{
    let mut result = Vec::new();
    let len1 = list1.len();
    let len2 = list2.len();

    let mut index1 = 0;
    let mut index2 = 0;

    while index1 < len1 && index2 < len2 {
        if list1[index1] < list2[index2] {
            result.push(list1[index1].clone());
            index1 += 1;
        } else {
            result.push(list2[index2].clone());
            index2 += 1;
        }
    }

    if index1 == len1 {
        for rest in index2..len2 {
            result.push(list2[rest].clone());
        }
    } else if index2 == len2 {
        for rest in index1..len1 {
            result.push(list1[rest].clone());
        }
    }

    result
}
