/*  SHAPES AREA CALCULATIONS

    The goal of this script is to:
        - Define a general enum "shape" containing all possible shapes (circle and rectangle).
        - Use `impl` to define the function 'area' for all shapes.
        - Create a structure 'Square' which will reuse all the code from 'Rectangle'
*/
use std::f64::consts;

// Definition of all general base shapes:
enum Shape {
    Circle { radius: u32 },
    Rectangle { width: u32, height: u32 },
}

// Implementation of the logic to calculate the areas:
impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Rectangle { width, height } => (width * height) as f64,
            Shape::Circle { radius } => (*radius as f64).powf(2.0) * consts::PI,
        }
    }
}

// Square definition -> A squares is a subcase of a rectangle.
struct Square {
    side: u32,
}

impl Square {
    fn area(&self) -> f64 {
        Shape::Rectangle {
            width: self.side,
            height: self.side,
        }
        .area()
    }
}

fn main() {
    // Rectangle:
    let rect = Shape::Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} squared pixels.",
        rect.area()
    );

    // Square:
    let sq = Square { side: 20 };
    println!("The area of the square is {} squared pixels.", sq.area());

    let circ = Shape::Circle { radius: 25 };
    println!("The area of the circle is {} squared pixels.", circ.area())
}
