fn main() {
    let width1 = 30;
    let height1 = 50;

    let rect1: (u32, u32) = (30, 50);

    let rectangle_struct = Rectangle {
    	width: 30,
    	height: 50
    };

    println!("The area of a rectangle is {:?} square pixels", 
    	area(width1, height1)
    );

    println!(
    	"The area of a rectangle is {:?} square pixels", 
    	area_by_tuple(rect1)
    );

    println!(
    	"The area of a rectangle is {:?} square pixels", 
    	area_by_struct(&rectangle_struct)
    );
}

struct Rectangle {
	width: u32,
	height: u32
}

fn area(width: u32, height: u32) -> u32 {
	width * height
}

fn area_by_tuple(dimension: (u32, u32)) -> u32 {
	dimension.0 * dimension.1
}

fn area_by_struct(rectangle: &Rectangle) -> u32 {
	rectangle.width * rectangle.height
}