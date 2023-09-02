#![allow(dead_code)]

use std::fmt::Display;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(top_left: {}, bottom_right: {})",
            self.top_left, self.bottom_right
        )
    }
}

fn rect_area(rect: &Rectangle) -> f32 {
    let Rectangle {
        top_left: Point { x: x1, y: y1 },
        bottom_right: Point { x: x2, y: y2 },
    } = rect;

    ((x2 - x1) * (y1 - y2)).abs()
}

fn square(point: &Point, length: f32) -> Rectangle {
    let Point { x, y } = point;
    Rectangle {
        top_left: Point { x: *x, y: *y },
        bottom_right: Point {
            x: *x + length,
            y: *y + length,
        },
    }
}

pub fn test() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    let point: Point = Point { x: 10.3, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);

    let bottom_right = Point { x: 5.2, ..point };
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let _rectangle = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: Point { x: 100.0, y: 100.0 },
    };
    let area = rect_area(&_rectangle);
    println!("rectangle: {}, area: {}", _rectangle, area);
    println!("square: {}", square(&point, 10.0));

    let _unit = Unit;
    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);
}
