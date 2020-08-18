mod draw_window;
mod triangle;
mod point;
mod tests;

use std::io;
use float_cmp::ApproxEq;
use std::str::FromStr;

use crate::point::Point;
use crate::triangle::Triangle;

fn main() {
    println!("I want to calc your triangle");

    let mut triangle_sides = [0.0; 3];

    for side in triangle_sides.iter_mut() {
        *side = handle_side();
    }

    if have_negative_sides(&triangle_sides) {
        println!("Triangle cannot have negative sides");
        return
    }

    if have_zero_sides(&triangle_sides) {
        println!("Triangle cannot have zero sides");
        return
    }

    if !can_construct_triangle(&triangle_sides[0], &triangle_sides[1], &triangle_sides[2]) {
        println!("This is not a triangle!");
        return
    }

    println!("Congrats! You are a triangle!");

    let mut triangle = Triangle {
        a: Point {x: 0.0, y: 0.0},
        b: Point {x: 0.0, y: 0.0},
        c: Point {x: 0.0, y: 0.0},
        ac_length: triangle_sides[0],
        ab_length: triangle_sides[1],
        cb_length: triangle_sides[2],
    };

    triangle.normalize_triangle();
    triangle.c = Point {x: triangle.ac_length, y: 0.0};
    triangle.b = triangle.calc_b_point();

    triangle.draw();
}

fn handle_side() -> f32 {
    println!("Print side side");

    let mut side = String::new();

    io::stdin()
        .read_line(&mut side)
        .expect("Failed to read, try again");

    f32::from_str(side.trim()).expect("You typed something, but not a number :(")
}

fn have_negative_sides(sides: &[f32; 3]) -> bool {
    for side in sides.iter() {
        if *side < 0.0 {
            return true
        }
    }

    false
}

fn have_zero_sides(sides: &[f32; 3]) -> bool {
    for side in sides.iter() {
        if ApproxEq::approx_eq( *side, 0.0, (0.0, 2)) {
            return true
        }
    }

    false
}

fn can_construct_triangle(a: &f32, b: &f32, c: &f32) -> bool {
    return *a + *b > *c && *a + *c > *b && *b + *c > *a;
}