use std::any::Any;

trait Shape {
    fn area(&self) -> f64;
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
fn test_shape() {
    let shapes: Vec<Box<dyn Any>> = vec![
        Box::new(Rectangle { width: 3.0, height: 4.0 }),
        Box::new(Circle { radius: 2.5 }),
    ];

    for shape in shapes {
        if let Some(rectangle) = shape.downcast_ref::<Rectangle>() {
            println!("Rectangle: width={}, height={}, area={}", rectangle.width, rectangle.height, rectangle.area());
        } else if let Some(circle) = shape.downcast_ref::<Circle>() {
            println!("Circle: radius={}, area={}", circle.radius, circle.area());
        }
    }
}
}