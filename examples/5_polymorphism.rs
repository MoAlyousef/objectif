use objectif::{call_method, inherits, table_init, super_init, Object};
use std::mem::transmute;

#[inherits(Object)]
struct Shape {
    parent: Object,
}

impl Shape {
    pub fn area(&self) -> f64 {
        0.0
    }
}

impl Default for Shape {
    fn default() -> Self {
        table_init! {
            Shape,
            "area": area,
        };
        Self {
            parent: super_init![Object::default()],
        }
    }
}

#[inherits(Shape)]
struct Rect {
    parent: Shape,
    x: f64,
    y: f64,
}

impl Rect {
    pub fn area(&self) -> f64 {
        self.x * self.y
    }
}

impl Rect {
    fn new(x: f64, y: f64) -> Self {
        table_init! {
            Rect,
            "area": area,
        }
        Self {
            parent: super_init![Shape::default()],
            x,
            y,
        }
    }
}

#[inherits(Rect)]
struct Square {
    parent: Rect,
}

impl Square {
    fn new(x: f64) -> Self {
        table_init! {
            Square,
        }
        Self {
            parent: super_init![Rect::new(x, x)],
        }
    }
}

#[inherits(Shape)]
struct Circle {
    parent: Shape,
    r: f64,
}

impl Circle {
    pub fn area(&self) -> f64 {
        self.r * self.r * 3.14
    }
}

impl Circle {
    fn new(r: f64) -> Self {
        table_init! {
            Circle,
            "area": area,
        }
        Self {
            parent: super_init![Shape::default()],
            r,
        }
    }
}

fn main() {
    let myrect = Rect::new(4., 5.);
    let mysquare = Square::new(4.);
    let mycircle = Circle::new(4.);

    let mut v: Vec<Box<Shape>> = vec![];
    unsafe {
        v.push(transmute(Box::new(myrect)));
        v.push(transmute(Box::new(mysquare)));
        v.push(transmute(Box::new(mycircle)));
    }
    for elem in v {
        let elem_area: f64 = unsafe { call_method![*elem, area].unwrap() };
        println!("element area: {elem_area}");
    }
}
