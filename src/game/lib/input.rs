use std::io;

pub fn read_line(message: String) -> String {
    println!("{}", message);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

pub fn read_int(message: String) -> u32 {
    println!("{}", message);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: u32 = input.trim().parse().expect("Please type a number!");
    input
}

pub fn read_float(message: String) -> f32 {
    println!("{}", message);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: f32 = input.trim().parse().expect("Please type a number!");
    input
}

pub fn yes_no(message: String) -> bool {
    println!("{}", message);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: String = input.trim().to_lowercase().parse().expect("Please type Yes/No or Y/N");
    if input == "y" || input == "yes" {
        return true;
    } else if input == "n" || input == "no" {
        return false;
    } else {
        println!("Please type Yes/No or Y/N");
        yes_no(message)
    }
}
