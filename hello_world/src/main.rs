use std::io;

fn main() {
    println!("Enter a number!");
    let mut data = String::new();

    io::stdin().read_line(&mut data).expect("Error");

    println!("You say: {}", data);
}
