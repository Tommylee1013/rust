use std::io; // standard input output과 동일한 기능

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new(); // let mut은 변경가능한 변수를 선언하는 것
    io::stdin().read_line(&mut guess).expect("Failed to read line"); // &은 참조연산

    println!("You guessed: {}", guess);
}
