#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}


#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}



fn main() {
    let home = IpAddr::V4(String::from("127.0.01"));

    let loopback = IpAddr::V6(String::from("::1"));

    let msg = Message::Move {x: 2, y: 3};


    println!("{:#?} {:#?}", home, loopback);

    println!("{:#?}", msg);
}
