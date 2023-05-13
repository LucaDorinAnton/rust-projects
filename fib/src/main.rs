use std::io;

fn main() {
    println!("Printing the first 10 Fibonacci numbers");

    for i in 1..11 {
        println!("{i}: {}", fib(i));
    }

    let mut num = String::new();

    println!("Which Fibonacci number would you like to see?");

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");

    let num: i32 = num
        .trim()
        .parse()
        .expect("Please type a number");

    println!("{num}: {}", fib(num))
}


fn fib(n: i32) -> i32 {

    let mut i = 1;
    let mut cur = 1;
    let mut prev = 1;

    while i < n {
        let next = cur + prev;
        prev = cur;
        cur = next;
        i += 1;
    }

    cur
}
