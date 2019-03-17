fn main() {
    println!("Hello, world!");
    rader_function(4, 8);

    digital();

    let raise: u8 = plus_three(25);
    println!("Raise value: {}", raise);
}

fn rader_function(bail: u8, x: u8) {
	println!("Radar Plus entity: {} x {}", bail, x);
}

fn digital() {
	println!("Hello,Nerd");
	let x = 12;
	let s = {
		let x = x * 5 + twenty();
		x + 1
	};
	println!("{:?} {}", x, s);
}

fn twenty() -> i32 {
	20
}

fn plus_three(x: u8) -> u8{
	x + 3
}
