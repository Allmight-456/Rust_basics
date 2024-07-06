// Clone ,Copy , Structs
#[derive(Copy, Clone)]             //It's a way to automatically generate implementations 
                                   //of certain traits for your structs or enums.

//Custom data types that group related data. 
struct Rectangle {
    width: u32,
    height: u32,
}

// A way to define methods for structs or enums.
impl Rectangle {
    // This is optional but often useful
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height  // Implicit return ,without ; by default the last expression is returned
    }

    fn perimeter(&self) -> u32 {
        return 2 * (self.width + self.height);  // Explicit return
        // 2 * (self.width + self.height)
    }
}

fn main() {
    // Using new
    let rect1 = Rectangle::new(30, 50);
    
    // Direct instantiation
    let rect2 = Rectangle { width: 10, height: 20 };

    println!("Area of rect1: {}", rect1.area());
    println!("Perimeter of rect2: {}", rect2.perimeter());

    // Copy in action
    let rect3 = rect1;  // rect1 is copied, not moved because of [derive(Copy,Clone)]
    println!("Area of rect1 (still accessible): {}", rect1.area());
    println!("Area of rect3: {}", rect3.area());
}


#[allow(dead_code)]
//clone trait in rust needs to be generated
#[derive(Clone)]
struct Person {
    name: String,
    age: u32,
}
#[allow(dead_code)]
fn clone() {
    let p1 = Person { name: String::from("Alice"), age: 30 };
    let p2 = p1.clone();
    
    println!("{} is {} years old", p1.name, p1.age);
    println!("{} is {} years old", p2.name, p2.age);
}