fn main() {
    let s = "Say, Hi";
    //s.push_str(" to Yelsin");
    let mut saver = String::from("Do you ");
    saver.push_str("Love Rust?");
    println!("{:?}", s);
    println!("{:?}", saver);

    let day = String::from("Jummat!");
    let greet = day;
    println!("Happy {:?}", greet);

    let deep_copy = String::from("Deep copy using clone");
    let d_copy = deep_copy.clone();
    println!("{:?}", d_copy);

    let x = 34;
    move_string(deep_copy);
    copy_integer(x);
    println!("{:?}", x);

    let inivate = make_string();
    println!("{:?}", inivate);

    let sara = braga("RingMax".to_string());
    println!("{:?}", sara);
    println!("{:?}", braga("Fuck u".to_string()));

    let pure = calculate_length("Read me now".to_string());
    println!("String: {:?}, length: {}", pure.0, pure.1);
}

fn move_string(some_string: String) {
	println!("{:?}", some_string);
}

fn copy_integer(some_integer: i32) {
	println!("{:?}", some_integer);
}

fn make_string() -> String {
	let r = String::from("Name: Yelsin");
	r
}

fn braga(allow: String) -> String {
	allow
}

fn calculate_length(bale: String) -> (String, usize) {
	let length = bale.len();
	(bale, length)
}
