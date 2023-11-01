use std::io::{Write, stdout, stdin };

pub fn print(text: &str) {
    print!("{text}");

    stdout().flush().unwrap();
}

pub fn read_string() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    return input.trim().parse().unwrap();
}

pub fn read_num() -> Option<i32> {
    let input = read_string();

    return match input.parse::<i32>() {
        Ok(num) => Some(num),
        Err(err) => {
            println!("{}: {}", err, input);
            return None;
        }
    };
}