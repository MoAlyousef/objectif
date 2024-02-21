use objectif::{call_method, inherits, super_init, table_init, Object};

#[derive(Clone)]
#[inherits(Object)]
struct MyObject {
    parent: Object,
    col: u32,
    opacity: f32,
}

impl MyObject {
    pub fn color(&self) -> u32 {
        self.col
    }
    pub fn set_color(&mut self, col: u32) {
        self.col = col;
    }
    pub fn set_color_with_opacity(&mut self, col: u32, opacity: f32) {
        self.col = col;
        self.opacity = opacity;
    }
}

impl Default for MyObject {
    fn default() -> Self {
        table_init! {
            MyObject,
            "color": color,
            "color:": set_color,
            "color:opacity:": set_color_with_opacity,
        };
        Self {
            parent: super_init![Object::default()],
            col: 0xffffff,
            opacity: 1.0,
        }
    }
}

fn main() {
    let myobj = MyObject::default();
    let _: () = unsafe { call_method![myobj, color:0xff0000 opacity:0.5].unwrap() };
    let col: u32 = unsafe { call_method![myobj, color].unwrap() };
    println!("myobj's color: {:#08x}", col);
}
