use objectif::{call_method, inherits, Object};

#[derive(Default)]
#[inherits(Object)]
struct MyObject {
    parent: Object,
}

fn main() {
    let myobj = MyObject::default();
    let val: bool = unsafe { call_method![myobj, has_method:"has_method:"].unwrap() };
    println!("Has method: \"has_method:\": {}", val);
}
