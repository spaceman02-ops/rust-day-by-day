fn main() {
    println!("{} degrees F is {} degrees C", 35.00, f_to_c(35.00));
    println!(
        "The Fibonacci number at position {} is {}",
        35,
        fibonacci(35)
    );
    twelve_days_of_christmas()
}

fn f_to_c(f: f32) -> f32 {
    return (f - 32.00) * (5.0 / 9.0);
}

fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

fn twelve_days_of_christmas() {
    let seq = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let gifts = [
        "12 drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five golden rings (five golden rings)",
        "Four calling birds",
        "Three French hens",
        "Two turtle-doves",
        "a partridge in a pear tree",
    ];
    for number in 0..12 {
        println!(
            "On the {} day of Christmas my true love gave to me {}",
            seq[number],
            gifts[11 - number]
        );
        for n in (0..number).rev() {
            if n == 0 {
                print!("And ")
            }
            println!("{}", gifts[11 - n])
        }
    }
}
//We're doing it!
