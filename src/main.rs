mod copy;
use copy::*;
use copy::mytrait::*;

fn main() {
    println!("Hello, world!");

    let a: &dyn Trait = &42_u8;
    let b: &dyn Trait = &String::from("hello");

    let number: u8 = *unsafe { a.downcast::<u8>() };
    let number1: u8 = *unsafe { b.downcast::<u8>() };
    let text: &str = unsafe { b.downcast::<String>() };

    println!("{} {} {}", number, number1, text);
}