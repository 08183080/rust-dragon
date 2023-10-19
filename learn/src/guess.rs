use std::io;

fn main() {
    println!("please input the number you want to guess: ");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("failed to read");
    println!("your guess: {guess}");
}