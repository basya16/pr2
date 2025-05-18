#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rectangle {
    a: Point, // верхній лівий
    b: Point, // нижній правий
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

use std::collections::HashSet;

fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut points = HashSet::new();

    for rect in xs {
        for x in rect.a.x..rect.b.x {
            for y in rect.b.y..rect.a.y {
                points.insert((x, y));
            }
        }
    }

    points.len() as i32
}

fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
}

fn main() {
    area_occupied_test();
    println!("Test passed!");
}
