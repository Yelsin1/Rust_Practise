fn main() {
    println!("Hello, world!");
    println!("You only type in {:?} words", calculate_length(&"Morogbo".to_string()));
    println!("{:#?}", make_change(&mut"Look at ".to_string()));
}

fn calculate_length(a: &String) -> usize {
	a.len()
}

fn make_change(a: &mut String) -> String {
    a.push_str(" Serial");
    a.to_string()
}

