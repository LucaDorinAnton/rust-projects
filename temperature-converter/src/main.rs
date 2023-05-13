use std::io;

fn main() {
    println!("Input a temperature in Fahrenheit.");

    let mut f = String::new();

    io::stdin()
        .read_line(&mut f)
        .expect("Failed to read line");

    let f_n: f64 = f.
        trim()
        .parse()
        .expect("Please type a number");

    println!("You asked to convert {f_n}°F to Celsius");

    let c_n: f64 = (f_n - 32.0) * (5.0 / 9.0);

    println!("{:.2}°F is {:.2}°C", f_n, c_n);

}
