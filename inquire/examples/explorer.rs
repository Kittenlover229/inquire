use inquire::Explorer;

pub fn main() {
    let path = Explorer::new("Hello, world!").prompt().unwrap();
    println!("{path:?}")
}
