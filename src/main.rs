// use std::io;
/*
// Note: Gussing Game
use std::{cmp::Ordering, io};

use rand::RngExt;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::rng().random_range(1..=100);

    loop {
        println!("please input your guess:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Your guess: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
*/

/* Note: immituble & mutable
fn main() {
    let mut  x: u8 = 55;
    println!("This is number: {}",x);
    x=56;
    println!("This is number: {}",x);
}*/

// Note: tuples type
/*
fn main() {
    let emp_info: (&str, u8) = ("Jim", 15);

    println!("your employer name: {} and age: {}", emp_info.0, emp_info.1);
    // distructuring
    let (employee_name, employee_age) = emp_info;
    println!(
        "your employer name: {} and age: {}",
        employee_name, employee_age
    );
}
*/


// Note: Function how it work
fn main() {
    fun_work();
    item_print(5);

    let num1: u8 = 10;
    let num2: u8 = 20;
    let result: u8 = add(num1, num2);
    print!("Summation = {result}")
}
fn fun_work() {
    println!("This try to function how it works.");
}
fn item_print(item: u8) {
    println!("my item number: {item}");
}
fn add(item1: u8, item2: u8) -> u8 {
    return item1 + item2;
}
