const WELCOME_MSG: &str = "Welcome to RustScape!";

fn main() {
    // Clears screen (since it's not obvious, I mean, what is this?)
    print!("\x1B[2J\x1B[1;1H");

    println!("{}", WELCOME_MSG);
}
