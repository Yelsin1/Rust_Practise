fn main() {
    let rect1 = Rectangle { width: 30, height: 50};
    let rect2 = Rectangle { width: 10, height: 40};
    let rect3 = Rectangle { width: 60, height: 45};
    println!("{:?}", rect1.area1(23));

    println!("{:#?}", rect1.area());
    println!("rect1 can hold rect2? {:?}", rect1.can_hold(&rect2));
    println!("rect1 can hold rect3? {:?}", rect1.can_hold(&rect3));
    println!("rect3 can hold rect1? {:?}", rect3.can_hold(&rect1));
    println!("rect3 can hold rect2? {:?}", rect3.can_hold(&rect2));
    println!("{:#?}", rect3.area());


}

#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32
}

impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	}

	fn area1(&self, bonus: u32) -> u32 {
		self.width * self.height + bonus
	}

	fn can_hold(&self, rectangle: &Rectangle) -> bool {
		self.width > rectangle.width && self.height > rectangle.height
	}
}
