use std::collections::HashMap;


fn median(v: &Vec<i32>) -> i32 {
    let mut vec = v.to_vec();
    vec.sort();
    let cnt = vec.len();
    vec[cnt / 2]
}


fn mode(v: &Vec<i32>) -> i32 {

    let mut map = HashMap::new(); 
    let mut max = 0;


    for i in v {
        let c = map.entry(i).or_insert(0);
        *c += 1;
        if *c > max {
            max = *i;
        }
    }   
    max
}


fn main() {
    let v = vec![1,2,3,4,5,2, 5, 5];

    println!("The mode of {:?} is {}", v, mode(&v));

    println!("The median of {:?} is {}", v, median(&v));
}
