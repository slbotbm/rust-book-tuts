fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    if number < 5 {
        println!("condition was true!");
    } else {
        println!("Condition was false!");
    }
    // loop {
    //     println!("again!");
    // }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }

    println!("LIFTOFF!");

    for element in a {
        println!(
            "{element} degree celsius is {} degree fahrenheit.",
            celsius_to_fahrenheit(element),
        );
        println!(
            "The {element}th fibonacci number is {}",
            fibonacci(element as u64)
        );
    }

    twelve_days_of_christmas()
}

fn celsius_to_fahrenheit(c: i32) -> f32 {
    (c as f32 * 9.0 / 5.0) + 32.0
}

fn fibonacci(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;
    for _ in 1..n + 1 {
        let c = a + b;
        a = b;
        b = c;
    }
    return a;
}

fn twelve_days_of_christmas() {
    const VERSES: [&str; 11] = [
        "Two turtle doves, ",
        "Three French hens, ",
        "Four calling birds,",
        "Five golden rings, ",
        "Six geese a-laying,",
        "Seven swans a-swimming, ",
        "Eight maids a-milking,",
        "Nine ladies dancing, ",
        "Ten lords a-leaping, ",
        "Eleven pipers piping, ",
        "Twelve drummers drumming, ",
    ];
    for day in 0..12 {
        println!("\n");
        println!(
            "On the {} day of Christmas, \nmy true love sent to me",
            day + 1
        );
        for i in (0..day).rev() {
            println!("{}", VERSES[i]);
        }
        if day > 0 {
            println!("And a partridge in a pear tree");
        } else {
            println!("A partridge in a pear tree");
        }
    }
}
