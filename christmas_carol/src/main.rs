fn main() {
    println!("On the {} day of Christmas my true love sent to me", "first");
    println!("A partridge in a pear tree.");

    for i in 1..12 {
        println!();
        sing(i);
    }
}

fn sing(n: i8) {
    let messages = [
        "And a partridge in a pear tree", "Two turtle doves",
        "Three French hens", "Four calling birds", "Five gold rings",
        "Six geese a-laying", "Seven swans a-swimming",
        "Eight maids a-milking", "Nine ladies dancing", "Ten lords a-leaping",
        "Eleven pipers piping", "Twelve drummers drumming"
    ];

    let days = [
        "first", "second", "third", "fourth", "fifth",  "sixth", "seventh",
        "eighth", "ninth", "tenth", 
        "eleventh", "twelfth"
    ];

    println!("On the {} day of Christmas my true love sent to me",
        days[n as usize]);

    for i in (0..=n).rev() {
        print!("{}", messages[i as usize]);
        println!("{}", if i == 0 { '.' } else { ',' });
    }
}
