fn main() {
	// immutable
    let x = 5;
    println!("the value of x is: {}", x);

    // mutable
    let mut p = 3;
    println!("The value of p is: {}", p);
    p = 12;
    println!("The value of p is: {}", p);

    // constant
    const MONTHS : u8 = 12;
    println!("We only have {} months in a year", MONTHS);

    // shadowing
    let d = 12;
    println!("the value of d is: {}", d);
    let d = d + 5;
    println!("the value of d is: {}", d);
    let d = d * 4;
    println!("the value of d is: {}", d);

    let spaces = "     ";
    let spaces = spaces.len();
    println!("space count: {}", spaces);

    // data types
    let _retain: u16 = "32".parse().expect("Not a number");
    println!("{}", _retain);

    // Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters. 

    // compound types: tuple and arrays

    // Tuples

    let tup: (u32, f32, u8) = (127_654, 45.78, 87);
    println!("access first element: {}", tup.0);
    println!("access second element: {}", tup.1);
    println!("access third element: {}", tup.2);

    let song = (12, 678.7, 'A');
    let (x, y, z) = song;
    println!("access x variable: {}", x);
    println!("access y variable: {}", y);
    println!("access z variable: {}", z);

    // Arrays
    let _a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    println!("months count: {}", months.len());

    let sign: [f32; 3] = [12.6, 86.55, 0.67];
    let first = sign[0];
    println!("First element of an array: {}", first);
    println!("second: {}", sign[1]);
}
