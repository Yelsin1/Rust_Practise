fn main() {
    println!("Hello, world!");

    // CRO-7960-ZOHY

    let four = IpAddressKind::V4;
    let six = IpAddressKind::V6;

    // Using enums with structs
    let home = IpAddress {
    	kind: IpAddressKind::V6, 
    	address: String::from("127.0.0.1")
    };

    let _reduce = Ipaddr::V6(String::from("::1"));

    let _deal = Message::Move{x: 23, y: 45};
    let get_string = Message::Write(String::from("Rida Network"));
    println!("{:?}", get_string.town_cryer());
    Some("somePay");
}

enum IpAddressKind {
	V4,
	V6
}

struct IpAddress {
	kind: IpAddressKind,
	address: String
}

// Creating enum with value holder 
enum Ipaddr {
	V4(u32, u32, u32, u32),
	V6(String),
	V8(u32, u32, u32)
}

enum Message {
	Quit,
	Move{x: i32, y: i32},
	Write(String),
	ChangeColor(i32, i32, i32)
}

impl Message {
	fn town_cryer(&self) {
		//println!("Panic!");
	}
}

enum Option<T> {
	Some(T),
	None
}