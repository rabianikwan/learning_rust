
// struct tipe data khusus yang memungkinkan kita membungkus
// beberapa nilai terkait atau grup yang bermakna (blueprint, constructor js)
    
struct User {
        name: String,
        age: u8,
        active: bool,
        sign_in_count: u64
}

struct Rectangle {
    width: u32,
    height: u32,
}
// tuple struct
struct Point (i32, i32);

// unit like struct
struct Unit;

// method struct 

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
}

fn main() {
    let user1 =  User {
        name: String::from("John"),
        age: 30,
        active: true,
        sign_in_count: 1,
    };
    println!("Hello {}", user1.name);
    
    // make mutable all variables in struct
    let mut user2 = User {
        name: String::from("John"),
        age: 30,
        active: true,
        sign_in_count: 1,
    };
    user2.age = 40;
    user2.name = String::from("Jane");
    println!("Hello {}", user2.name);
    
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Rectangle area: {}", area(rect1.width, rect1.height));
    
    // menggunakan method struct
    println!("Rectangle area: {:?}", rect1.area());
    // tuple struct
    let score_point = Point (5, 10);
    println!("Point: {:?}", score_point.0);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
