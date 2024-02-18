use objectif::{add_class_method, call_method, inherits, Object};

#[derive(Default)]
#[inherits(Object)]
struct MyObject {
    parent: Object,
    num: i32,
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
