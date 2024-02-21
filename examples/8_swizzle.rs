use objectif::{add_class_method, call_method, inherits, table_init, super_init, Object};

#[inherits(Object)]
struct MyObject {
    parent: Object,
    num: i32,
}

impl MyObject {
    pub fn number(&self) -> i32 {
        self.num
    }
}

impl Default for MyObject {
    fn default() -> Self {
        table_init!(MyObject,);
        Self {
            parent: super_init![Object::default()],
            num: 5,
        }
    }
}

fn new_number(obj: *const MyObject) -> i32 {
    unsafe { (*obj).num + 2 }
}

fn main() {
    unsafe {
        add_class_method![MyObject, "number", MyObject::number];
        let myobj = MyObject::default();
        let num: i32 = call_method![myobj, number].unwrap();
        println!("Number is {num}");

        let old_fn = add_class_method![MyObject, "number", new_number].unwrap();
        let myobj2 = MyObject::default();
        let num: i32 = call_method![myobj2, number].unwrap();
        println!("Number is {num}");

        // We can use the returned fn pointer to invoke the old method on the same object
        let old_fn: fn(*const Object) -> i32 = std::mem::transmute(old_fn);
        println!("Old number is {}", (old_fn)(&*myobj2));
    }
}
