use std::collections::HashMap;

fn main() {
    println!("{}", median(&mut vec![1,98,6,44,56,23,78,5555,334,90,123,543,765,123,45,67]));
    println!("{}", mode(vec![7,7,1,98,34,2223,989,4,5,22,98,98,1,7,7,2225,2224,100,1001,100,100,100,100,2223]))
}

fn median(vector: &mut Vec<i32>) -> i32 {
    vector.sort_by(|a, b| a.cmp(b));
    vector[vector.len() / 2]
}
fn mode(vector: Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for i in vector {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    let mut high = 0;
    let mut key = 0;
    
    for (k, v) in map.iter() {
        if v > &high {
            high = *v;
            key = *k;
        }
    }
    key
}
