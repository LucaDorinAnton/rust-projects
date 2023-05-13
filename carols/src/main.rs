fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eight", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let verses = [
        "And a partridge in a pear tree",
        "Two turtledoves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings (five golden rings)",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    println!("'Twelve days of Christmas' by Bing Crosby: \n\n");

    for i in 0..12 {
        println!(
            "On the {} day of Christmas, my true love sent to me",
            days[i]
        );
        for j in (0..=i).rev() {
            if i == 0 && j == 0 {
                println!("A partridge in a pear tree");
            } else {
                println!("{}", verses[j]);
            }
        }

        println!();
    }

    println!("{}", verses[0]);
}
