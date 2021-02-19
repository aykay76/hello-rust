fn main() {
    let mut guess = String::new();
    let greeting = "What's your name?";

    println!("{}", greeting);

    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed somehow");

    println!("Hello, {}", guess);
}
