use objectif::{inherits, is_child_of, is_instance_of, super_init, table_init, Object};

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
