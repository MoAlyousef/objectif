use objectif::{call_method, inherits, super_call, super_init, table_init, Object};

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
        table_init! {
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
        table_init! {
            Rect,
            "draw": draw,
        };
        Self {
            parent: super_init![Shape::default()],
            x,
            y,
        }
    }
    pub fn draw(&self) {
        // can perform a static call
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
        table_init! {
            Circle,
            "draw": draw,
        };
        Self {
            parent: super_init![Shape::default()],
            r,
        }
    }
    pub fn draw(&self) {
        // or a dynamic call
        let _: () = unsafe { super_call![self.parent, draw].unwrap() };
    }
}

fn main() {
    let myrect = Rect::new(4., 5.);
    let _: () = unsafe { call_method![myrect, draw].unwrap() };

    let mycircle = Circle::new(4.);
    let _: () = unsafe { call_method![mycircle, draw].unwrap() };
}
