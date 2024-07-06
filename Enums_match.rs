// Custom types with a fixed set of variants, can contain data that you can define
// there can be only these many variants for a particular enum
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Square(f64),
}

//A way to define methods/Operations for structs or enums.
impl Shape {

    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Rectangle(length, breadth) => length * breadth,
            Shape::Square(side) => side * side,
        }
    }
    fn perimeter(&self)-> f64 {
        match self {
            Shape::Circle(radius) => 2.0 * std::f64::consts::PI * radius,        // add .0 to integers to make it float
            Shape::Rectangle(length, breadth) => 2.0* (length + breadth),
            Shape::Square(side) => 4.0 * side,
        }
    }
}

fn main(){
    let circle = Shape::Circle(5.00);
    let rectangle = Shape::Rectangle(4.00,6.00);
    let square = Shape::Square(3.00);

    println!("Area of Circle is : {} and perimeter is {:.2}", circle.area(),circle.perimeter());         // :.2 to give floating values upto two decimal places 
    println!("Area of Rectangle is : {} and perimeter is {:.2}", rectangle.area(),rectangle.perimeter());
    println!("Area of Square is : {} and perimeter is {:.2}", square.area(),square.perimeter());
}
