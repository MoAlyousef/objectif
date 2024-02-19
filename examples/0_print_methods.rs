use objectif::{inherits, init_table, super_init, Object};

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
    myobj.print_methods();
}
