use objectif::{call_method, inherits, super_call, super_init, table_init, Object};

#[inherits(Object)]
struct Shape {
    parent: Object,
}

impl Shape {
    pub fn draw(&self) {
        println!("Draw Shape");
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
        self.parent.draw();
        println!("Draw Rect");
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
        println!("Draw circle");
    }
}

#[inherits(Shape)]
struct Triangle {
    parent: Shape,
    b: f64,
    h: f64,
}

impl Triangle {
    fn new(b: f64, h: f64) -> Self {
        table_init! {
            Triangle,
            "draw": draw,
        };
        Self {
            parent: super_init![Shape::default()],
            b,
            h,
        }
    }
    pub fn draw(&self) {
        // or a dynamic call using any parent in the hierarchy
        let _: () = unsafe { super_call![Shape, self, draw].unwrap() };
        println!("Draw Triangle");
    }
}

fn main() {
    let myrect = Rect::new(4., 5.);
    let _: () = unsafe { call_method![myrect, draw].unwrap() };

    let mycircle = Circle::new(4.);
    let _: () = unsafe { call_method![mycircle, draw].unwrap() };

    let mytriangle = Triangle::new(4., 2.);
    let _: () = unsafe { call_method![mytriangle, draw].unwrap() };
}
