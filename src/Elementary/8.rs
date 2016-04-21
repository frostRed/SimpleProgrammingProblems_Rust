fn main() {
    use std::time::SystemTime;
    use std::u64;
    let max = u64::max_value();
    let mut con = Vec::new();

    let e = SystemTime::now();
    find_primer(max, &mut con);
    let ed = SystemTime::now();
    println!("time spend: {:?}", ed.duration_since(e).unwrap());

    match con.last() {
        Some(v) => println!("{}", v),
        None => println!("There is no primer."),
    }

}
fn find_primer(max: u64, con: &mut Vec<u64>) {
    for i in (1..).take(max as usize) {
        if i == 1 || i == 2 || i == 3 {
            con.push(i);
        } else {
            let mut flag = true;
            let top = (i as f64).sqrt() as u64;

            for j in con.into_iter().skip(1) {
                if *j > top {
                    break;
                }
                match i.checked_rem(*j) {
                    None => println!("Error: divisor is 0"),
                    Some(0) => flag = false,
                    Some(_) => (),
                }
            }

            if flag {
                con.push(i);
            }
        }
    }
}

fn find_primer_2(max: u64, con: &mut Vec<u64>) {
    for i in (1..).take(max as usize) {
        if i == 1 || i == 2 || i == 3 {
            con.push(i);
        } else {
            let mut flag = true;
            let mut top = (i as f64).sqrt() as u64;

            for j in con.into_iter().skip(1).filter(|x| x <= &&mut top) {
                match i.checked_rem(*j) {
                    None => println!("Error: divisor is 0"),
                    Some(0) => flag = false,
                    Some(_) => (),
                }
            }

            if flag {
                con.push(i);
            }
        }
    }
}
