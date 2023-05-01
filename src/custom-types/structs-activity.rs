// An attribute to hide warning for unused code. (sem isso, a compilacao gera erro nos codigos(vars) n sendo usadas)
#![allow(dead_code)]

#[derive(Debug)]

struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn react_area(point: Point, bottom_right: f32) -> Rectangle {
    // using destructuring
    let Point { x: _left_edge, y: top_edge } = point;

    let result = Rectangle {
        top_left: point,
        bottom_right: Point { x: bottom_right, y: top_edge },
    };

    // return
    result
}

fn main() {

    // Get the Reactangle
    let point = Point { x: 10.0, y: 10.0 };
    let rectangle = react_area(point, 12.0);

    // Rectangle data
    println!("{:?}, {:?}", rectangle.top_left, rectangle.bottom_right);
}