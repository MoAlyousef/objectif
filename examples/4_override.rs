use objectif::{call_method, inherits, init_table, super_init, Object};

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
        init_table! {
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
    fn new(x: f64, y: f64) -> Self {
        init_table! {
            Rect,
            "area": area,
        }
        Self {
            parent: super_init![Shape::default()],
            x,
            y
        }
    }
    pub fn area(&self) -> f64 {
        self.x * self.y
    }
}

#[inherits(Rect)]
struct Square {
    parent: Rect,
}

impl Square {
    fn new(x: f64) -> Self {
        init_table! {
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
    fn new(r: f64) -> Self {
        init_table! {
            Circle,
            "area": area,
        }
        Self {
            parent: super_init![Shape::default()],
            r
        }
    }
    pub fn area(&self) -> f64 {
        self.r * self.r * 3.14
    }
}

fn main() {
    let myrect = Rect::new(4., 5.);
    let rect_area: f64 = unsafe { call_method![myrect, area].unwrap() };
    println!("myrect's area: {rect_area}");

    let mysquare = Square::new(4.);
    let square_area: f64 = unsafe { call_method![mysquare, area].unwrap() };
    println!("mysquare's area: {square_area}");

    let mycircle = Circle::new(4.);
    let circle_area: f64 = unsafe { call_method![mycircle, area].unwrap() };
    println!("mycircle's area: {circle_area}");
}
