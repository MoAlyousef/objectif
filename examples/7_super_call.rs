use objectif::{call_method, inherits, init_table, super_init, Object};

#[inherits(Object)]
struct Shape {
    parent: Object,
}

impl Shape {
    pub fn draw(&self) {
        // do drawing
    }
}

impl Default for Shape {
    fn default() -> Self {
        init_table! {
            Shape,
            "draw": draw,
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
            "draw": draw,
        }
        Self {
            parent: super_init![Shape::default()],
            x,
            y
        }
    }
    pub fn draw(&self) {
        self.parent.draw()
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
            "draw": draw,
        }
        Self {
            parent: super_init![Shape::default()],
            r
        }
    }
    pub fn draw(&self) {
        self.parent.draw()
    }
}

fn main() {
    let myrect = Rect::new(4., 5.);
    let _: () = unsafe { call_method![myrect, draw].unwrap() };

    let mycircle = Circle::new(4.);
    let _: () = unsafe { call_method![mycircle, draw].unwrap() };
}
