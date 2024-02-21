use objectif::{call_method, inherits, table_init, super_init, Object};

#[inherits(Object)]
struct MyObject {
    parent: Object,
}

impl Default for MyObject {
    fn default() -> Self {
        table_init!(MyObject);
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
