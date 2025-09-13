mod shapes;
use shapes::shape::Shape;

mod circle;
use circle::Circle;

mod rectangle;
use rectangle::Rectangle;

mod helpers;
use helpers::{get_input_with_prompt, string_to_f64};

fn main() {

    let mut input: String = get_input_with_prompt("Enter the radius of the circle:");

    let radius: f64 = string_to_f64(input);

    let circle: Circle = Circle::new(radius);
    
    let mut area: f64 = circle.area();

    let mut perimeter: f64 = circle.perimeter();

    println!("Area of circle: {:.2}", area);

    println!("Perimeter of circle: {:.2}", perimeter);

    input = get_input_with_prompt("Enter the width of the rectangle:");

    let width: f64 = string_to_f64(input);

    input = get_input_with_prompt("Enter the height of the rectangle:");

    let height: f64 = string_to_f64(input);

    let rectangle: Rectangle = Rectangle::new(width,height);
    
    area = rectangle.area();

    perimeter = rectangle.perimeter();

    println!("Area of rectangle: {:.2}", area);

    println!("Perimeter of rectangle: {:.2}", perimeter);

}