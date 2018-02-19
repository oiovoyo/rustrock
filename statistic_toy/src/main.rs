use std::collections::HashMap;

fn main() {
    let mut v = [
        1, 65, 3, 88, 9, 12, 33, -1, 6, 5, 33, 1, 2, 3, 5, 6, 99, 0, 88
    ];
    v.sort();
    let l = v.len();
    println!("median:{:?}", v[l / 2]);
    let mut sum: i32 = 0;

    for i in v.into_iter() {
        sum = sum + i;
    }
    println!("average:{}", sum / l as i32);
    let mut map = HashMap::new();

    for i in v.into_iter() {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
    let mut maxcount: i32 = 0;
    let mut maxvalue: i32 = 0;
    for (k, v) in map {
        if maxcount < v {
            maxcount = v;
            maxvalue = *k;
        }
    }
    println!("modest {}, times {}", maxvalue, maxcount);
}
