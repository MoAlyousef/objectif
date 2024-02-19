use objectif::{call_method, inherits, init_table, super_init, Object};

#[inherits(Object)]
struct MyObject {
    parent: Object,
}

impl Default for MyObject {
    fn default() -> Self {
        init_table!(MyObject);
        Self {
            parent: super_init![Object::default()],
        }
    }
}

fn main() {
    let myobj = MyObject::default();
    let val: bool = unsafe { call_method![myobj, has_method:"has_method:"].unwrap() };
    println!("Has method: \"has_method:\": {}", val);
}
