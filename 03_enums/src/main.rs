enum Shape {
    Rectangle(f64, f64), // width, height
    Circle(f64),
    Square(f64),
}

fn main() {
    let rect = Shape::Rectangle(1.0, 2.0);
    println!("area of the rectangle is: {}", calculate_area(rect));
    let circle = Shape::Circle(2.0);
    println!("area of the circle is: {}", calculate_area(circle));
    let square = Shape::Square(2.0);
    println!("area of the circle is: {}", calculate_area(square));
}

fn calculate_area(shape: Shape) -> f64 {
    // pattern matching
    match shape {
        Shape::Rectangle(width, height) => width * height,
        Shape::Circle(radius) => 3.14 * radius * radius,
        Shape::Square(side_length) => side_length * side_length,
    } // implicit return
}
