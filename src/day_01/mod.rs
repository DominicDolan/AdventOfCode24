
pub fn main() {
    let x = 5;
    println!("Hello, world! {x}");
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}", guess);
}