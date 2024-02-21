use objectif::{add_class_method, call_method, inherits, table_init, super_init, Object};

#[inherits(Object)]
struct MyObject {
    parent: Object,
    num: i32,
}

impl Default for MyObject {
    fn default() -> Self {
        table_init!(MyObject);
        Self {
            parent: super_init![Object::default()],
            num: 5,
        }
    }
}

fn number(obj: *mut MyObject) -> i32 {
    unsafe { (*obj).num }
}

fn main() {
    unsafe {
        add_class_method![MyObject, "number", number];
        let myobj = MyObject::default();
        let num: i32 = call_method![myobj, number].unwrap();
        println!("Number is {num}");
    }
}
