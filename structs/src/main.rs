fn main() {
    
    let yelsin = User {
    	username: "hackflames".to_string(),
    	email: "yelsinlous@gmail.com".to_string(),
    	active: true,
    	sign_in_count: 1,
    };

    println!("--------------------------------");
    println!("{:?}", yelsin.username);
    println!("{:?}", yelsin.email);
    println!("{:?}", yelsin.active);
    println!("{:?}", yelsin.sign_in_count);

    let mut bale = User {
    	username: String::from("Garath Bale"),
    	email: "garath.bale@realmadrid.com".to_string(),
    	active: yelsin.active,
    	sign_in_count: yelsin.sign_in_count
    };

    println!("--------------------------------");
    println!("{:?}", bale.username);
    println!("{:?}", bale.email);
    println!("{:?}", bale.active);
    println!("{:?}", bale.sign_in_count);
    println!("--------------------------------");

    bale.username = String::from("Bale");
    println!("Username Changed: {:?}", bale.username);

    let modazab = build_user(String::from("Modazab"), String::from("modazab@vitogens.tech"));
    println!("--------------------------------");
    println!("{:?}", modazab.username);
    println!("{:?}", modazab.email);
    println!("{:?}", modazab.active);
    println!("{:?}", modazab.sign_in_count);
    println!("--------------------------------");

    // modazab.username = "CyberSadiq".to_string(); for mutable variables only

    let hazard = User {
    	username: String::from("Eden Hazard"),
    	email: "hazard.eden@chelseafc.com".to_string(),
    	..yelsin
    };

    println!("[{:?}, {:?}, {:?}, {:?}]", hazard.username, hazard.email, hazard.active, hazard.sign_in_count);

    // Tuple Struct
    let _black = Color(0, 0, 0);
    println!("{:?}", _black.0);

}

struct User {
	email: String,
	username: String,
	active: bool,
	sign_in_count: u64
}

fn build_user(name: String, email: String) -> User {
	User {
		username: name,
		email,
		active: true,
		sign_in_count: 1,
	}
}

struct Color(i32, i32, i32);
