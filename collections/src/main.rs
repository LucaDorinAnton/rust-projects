use std::collections::HashMap;


#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}



fn make_row() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("Row: {:#?}", row);
}



fn strings() {

    let mut s = String::from("foo");

    s.push_str("bar");

    println!("{s}");


    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    s = format!("{s1}-{s2}-{s3}");

    println!("{s}")

}


fn unicode() {
    let s = String::from("Лорем");
    for c in s.chars() {
        println!("{c}");
    }

    for b in s.bytes() {
        println!("{b}");
    }

}


fn hashmaps() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let team_score = scores.get(&team_name).copied().unwrap_or(0);

    println!("Team {team_name} has a score of {team_score}");


    for (key, value) in &scores {
        println!("Team {key} score {value}");
    }

    
    let k = "a";
    let v = "b";
    let mut map = HashMap::new();
    map.insert(k, v);

    println!("{k}");


    let k_str = String::from("a");
    let v_str = String::from("b");
    let mut map_str = HashMap::new();
    map_str.insert(k_str, v_str);

    // Will panic, as k_str has already been moved
    // println!("{k_str}");


}



fn maps_overwriting() {

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores)

}


fn entry_or_insert() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}


fn count_words() {
    let mut words = HashMap::new();


    for word in "hello world wonderful world".split_whitespace() {
        let count = words.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", words);
}


fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{i}");
    }


    make_row();

    strings();

    unicode();

    hashmaps();

    maps_overwriting();

    entry_or_insert();

    count_words();

}

