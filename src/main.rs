mod tests;

use std::io;
use float_cmp::ApproxEq;
use std::str::FromStr;

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

    println!("Congrats! You are a triangle!")
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