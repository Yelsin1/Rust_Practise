use std::io;

fn main() {
    let number = 3;

    if number < 5 {
    	println!("condition was true");
    } else {
    	println!("condition was flase");
    }

    if number != 0 {
    	println!("Number was something other than zero");
    }

    println!("Enter a number: ");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess: u16 =  guess.trim().parse().expect("Not a number");

    if guess % 4 == 0 {
    	println!("{} is divisible by 4", guess);
    } else if guess % 3 == 0 {
    	println!("{} is divisible by 3", guess);
    } else if guess % 2 == 0 {
    	println!("{} is divisible by 2", guess);
    } else {
    	println!("{} is not divisible by 4, 3 and 2", guess);
    }

    let store = if guess == 10 {
    	10
    } else {
    	20
    };
    println!("Store value: {}", store);

    loop {
    	println!("Again!");
    	break;
    }

    let mut counter = 0;
    let result = loop {
    	counter += 1;

    	if counter == 10 {
    		break counter * 2;
    	}
    };

    assert_eq!(result, 20);

    println!("result: {}", result);

    let mut seal = 0;

    while seal < 10 {
     	println!("{}, ", seal);

     	seal = seal + 1;
    }

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
    	println!("{:?}", a[index]);

    	index = index + 1;
    }

    for view in a.iter() {
    	println!("{:?}", view);
    }

    for num in (1..11).rev() {
    	println!("{:?} number", num);
    }
}
