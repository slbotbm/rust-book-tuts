fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}. ", s1, len);

    change(&mut s1);
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}. ", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        println!("{}", item);
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
