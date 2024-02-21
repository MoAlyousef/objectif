use objectif::{call_method, inherits, super_init, table_init, Object};

#[inherits(Object)]
struct MyObject {
    parent: Object,
    col: u32,
}

impl MyObject {
    pub fn print_color(&self) {
        println!("{}", self.col);
    }
    pub fn print_color_formatted(&self) {
        println!("{:#08x}", self.col);
    }
}

impl Default for MyObject {
    fn default() -> Self {
        table_init! {
            MyObject,
            "print_color": print_color,
        };
        Self {
            parent: super_init![Object::default()],
            col: 0xffffff,
        }
    }
}

// for capital hex output
fn print_color_formatted(obj: *const MyObject) {
    unsafe {
        println!("{:#08X}", (*obj).col);
    }
}

fn main() {
    let mut myobj = MyObject::default();
    let _: () = unsafe { call_method![myobj, print_color].unwrap() };
    unsafe {
        myobj.add_method("print_color", MyObject::print_color_formatted as *const ());
    }
    let _: () = unsafe { call_method![myobj, print_color].unwrap() };
    unsafe {
        myobj.add_method("print_color", print_color_formatted as *const ());
    }
    let _: () = unsafe { call_method![myobj, print_color].unwrap() };
}
