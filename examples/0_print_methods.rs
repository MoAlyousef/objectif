use objectif::{inherits, super_init, table_init, Object};

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
    myobj.print_methods();
}
