fn main() {
    println!("Hello, world!");
    println!("{:?}", first_word(&"Hello, Yelsin".to_string()));

    let mut me = String::from("I'm new to rust");
    let word = first_word(&me);
    me.clear(); // this empties the String, making it equal to ""
    println!("{:?}", word);

    let girl = String::from("Kafayat Olamide Alade");
    let first = &girl[0..7];
    let second = &girl[8..=14];
    println!("{:?} {:?} {:?}", girl, first, second);

    let third = &girl[..7];
    let len = girl.len();
    let forth = &girl[16..len];
    let fifth = &girl[16..];
    let sixth = &girl[0..len]; 
    let seventh = &girl[..];
    println!("{:?} {:?} {:?} {:?} {:?}", third, forth, fifth, sixth, seventh);

    println!("{:?}", show_first_word(&"Yelsin Buniyamin".to_string()));
}

fn first_word(s: &String) -> usize {
	let bytes = s.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return i;
		}
	}

	s.len()
}

fn show_first_word(s: &String) -> &str {
	let bytes = s.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[0..i];
		}
	}

	&s[..]
}
