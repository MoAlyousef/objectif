use once_cell::sync::Lazy;
use parking_lot::ReentrantMutex;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::sync::Arc;

pub type MapType = BTreeMap<&'static str, fn(*mut Object)>;

pub type RCellMapType = RefCell<MapType>;

pub type VTableInner = ReentrantMutex<RCellMapType>;

pub type VTable = Arc<VTableInner>;

pub type LazyVTable = Lazy<VTableInner>;

#[macro_export]
macro_rules! expr_as_underscore {
    ($e:expr) => {
        _
    };
}

#[macro_export]
macro_rules! call_method {
    ($obj:expr,$name:tt) => {
        {
            let val = $obj.vtable();
            let ret = match val.lock().borrow().get(stringify!($name)) {
                Some(v) => {
                    Some(
                        (std::mem::transmute::<_, fn(*mut $crate::Object) -> _>(*v as fn(_)))(
                            &$obj as *const _ as *mut $crate::Object,
                        )
                    )
                },
                None => None,
            };
            ret
        }
    };
    ($obj:expr, $($name:ident : $arg:expr)+) => {
        {
            let name = concat!($(stringify!($name), ':'),+);
            let val = $obj.vtable();
            let ret = match val.lock().borrow().get(name) {
                Some(v) => {
                    Some(
                        (std::mem::transmute::<_, fn(*mut $crate::Object, $($crate::expr_as_underscore!($arg)),+) -> _>(*v as fn(_)))(
                            &$obj as *const _ as *mut $crate::Object, $($arg,)+
                        )
                    )
                },
                None => None,
            };
            ret
        }
    };
}

#[macro_export]
macro_rules! add_class_method {
    ($i:ident, $sel:literal, $f:expr) => {
        $i::method_table()
            .lock()
            .borrow_mut()
            .insert($sel, std::mem::transmute($f as *const ()))
    };
}

#[macro_export]
macro_rules! define_class {
    ($name:ident:$parent:ident) => {
        impl std::ops::Deref for $name {
            type Target = $parent;
            fn deref(&self) -> &Self::Target {
                &self.parent
            }
        }
        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.parent
            }
        }
        unsafe impl Send for $name {}
        unsafe impl Sync for $name {}
    };
}

#[derive(Debug, Clone)]
pub struct Object {
    vtable: crate::VTable,
}

#[allow(non_upper_case_globals)]
static Object_METHOD_TABLE: LazyVTable = LazyVTable::new(|| unsafe {
    let mut map = BTreeMap::new();
    map.insert("is_object", std::mem::transmute(Object::is_object as *const ()));
    map.insert("has_method:", std::mem::transmute(Object::has_method as *const ()));
    VTableInner::new(RefCell::new(map))
});

impl Default for Object {
    fn default() -> Self {
        Lazy::force(&Object_METHOD_TABLE);
        let m = Lazy::get(&Object_METHOD_TABLE).unwrap();
        let vtable = VTable::new(VTableInner::new(RefCell::new(
            m.lock().clone().into_inner(),
        )));
        Self { vtable }
    }
}

impl Object {
    pub fn vtable(&self) -> VTable {
        return self.vtable.clone();
    }

    pub fn method_table() -> &'static LazyVTable {
        &Object_METHOD_TABLE
    }

    pub fn get_method(&self, name: &'static str) -> Option<fn(*mut Object)> {
        if let Some(f) = self.vtable().lock().borrow().get(name) {
            Some(*f)
        } else {
            None
        }
    }

    pub unsafe fn add_method(
        &mut self,
        name: &'static str,
        f: *const (),
    ) -> Option<fn(*mut Object)> {
        self.vtable().lock().borrow_mut().insert(name, std::mem::transmute(f))
    }

    pub unsafe fn try_add_method(&mut self, name: &'static str, f: *const ()) -> Result<(), &str> {
        let t = self.vtable();
        let t = t.lock();
        if t.borrow().contains_key(name) {
            Err("Key exists")
        } else {
            t.borrow_mut().insert(name, std::mem::transmute(f));
            Ok(())
        }
    }

    /// TODO these need to be macros
    pub unsafe fn try_add_class_method(name: &'static str, f: *const ()) -> Result<(), &str> {
        let t = Object_METHOD_TABLE.lock();
        if t.borrow().contains_key(name) {
            Err("Key exists")
        } else {
            t.borrow_mut().insert(name, std::mem::transmute(f));
            Ok(())
        }
    }

    fn has_method(&self, nm: &str) -> bool {
        self.vtable().lock().borrow().contains_key(nm)
    }

    fn is_object(&self) -> bool {
        true
    }
}
