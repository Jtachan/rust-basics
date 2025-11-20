/*
    Definition of two structures (Rectangle and Circle) and calculating their areas.
*/
use std::f64::consts;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        self.radius.powf(2.0) * consts::PI
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} squared pixels.",
        rect.area()
    );

    let circ = Circle { radius: 25.0 };
    println!("The area of the circle is {} squared pixels.", circ.area())
}
