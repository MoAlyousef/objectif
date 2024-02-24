use objectif::{inherits, super_init, table_init, is_instance_of, is_child_of, Object};

#[derive(Clone)]
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
    let val: bool = is_instance_of![myobj, Object];
    println!("myojb is instance of Object: {}", val);
    let val: bool = is_child_of![myobj, Object];
    println!("myojb is child of Object: {}", val);
}