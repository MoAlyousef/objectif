use objectif::{add_class_method, call_method, inherits, init_table, super_init, Object};

#[inherits(Object)]
struct MyObject {
    parent: Object,
    num: i32,
}

impl Default for MyObject {
    fn default() -> Self {
        init_table!(MyObject);
        Self {
            parent: super_init![Object::default()],
            num: 5,
        }
    }
}

fn print(obj: *mut MyObject) {
    unsafe {
        println!("{}", (*obj).num);
    }
}

fn main() {
    unsafe {
        add_class_method![Object, "print", print];
        let mut myobj = MyObject::default();
        myobj.num = 5;
        let _: () = call_method![myobj, print].unwrap();
    }
}
