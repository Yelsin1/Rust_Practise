fn main() {
	let mut counter: u32 = 0;

	// Loop
    loop {
    	println!("Yelsin is a rust developer: {}", counter);
    	
    	if counter == 1250 { // 4294967295
    		break;
    	}

    	counter += 1;
    }

    let mut digit = 10;

    while digit != 0 {
    	println!("counter: {:?}", digit);
    	digit -= 1;
    }

    // arrays
    let names: [&str; 3] = ["Yelsin", "Sadiq", "Olafemi"];
    let mut index = 0;

    while index < 3 {
    	println!("Names: {:?}", names[index]);
    	index += 1;
    }

    let mut s = 35;

    // loop in let statement
    let store = loop {
    	let mut total = 0;
    	total += s;

    	if s > 50 {
    		break s * 5 + total;
    	}
    	
    	s += 1;
    };

    println!("Final: {:?}", store);

    // Looping through array data
    for e in names.iter() {
    	println!("{:?}", e);
    }

    //range numbers
    for ave in (1..21).rev() {
    	println!("{:?}", ave);
    }

    // Looping through array and reversing data
    for e in names.iter().rev() {
    	println!("{:?}", e);
    }
}
