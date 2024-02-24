use once_cell::sync::Lazy;
use parking_lot::ReentrantMutex;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::sync::Arc;

#[doc(hidden)]
pub type OLazy<T, F = fn() -> T> = Lazy<T, F>;

#[doc(hidden)]
pub type MapType = BTreeMap<&'static str, fn(*mut Object)>;

#[doc(hidden)]
pub type Parents = Vec<std::any::TypeId>;

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct VTable {
    pub map: MapType,
    pub tids: Parents,
}

#[doc(hidden)]
pub type OVTable = Arc<ReentrantMutex<RefCell<VTable>>>;

#[doc(hidden)]
pub type RCellMapType = RefCell<MapType>;

#[doc(hidden)]
pub type VTableInner = ReentrantMutex<RCellMapType>;

#[doc(hidden)]
pub type LazyVTable = Lazy<VTableInner>;


#[doc(hidden)]
#[macro_export]
macro_rules! _expr_as_underscore {
    ($e:expr) => {
        _
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _call_method {
    ($obj:expr, $name:tt) => {
        {
            let val = $obj.vtable();
            let ret = match val.lock().borrow().map.get(stringify!($name)) {
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
            let ret = match val.lock().borrow().map.get(name) {
                Some(v) => {
                    Some(
                        (std::mem::transmute::<_, fn(*mut $crate::Object, $($crate::_expr_as_underscore!($arg)),+) -> _>(*v as fn(_)))(
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

#[doc(hidden)]
#[macro_export]
macro_rules! _add_class_method {
    ($i:ident, $sel:literal, $f:expr) => {
        $i::method_table()
            .lock()
            .borrow_mut()
            .insert($sel, std::mem::transmute($f as *const ()))
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _define_class {
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

#[doc(hidden)]
#[macro_export]
macro_rules! _table_init {
    ($obj:ident) => {
        {
            let mut map = $crate::MapType::default();
            $obj::method_table().lock().borrow_mut().extend(map)
        }
    };
    ($obj:ident, $($arg:literal : $name:ident,)*) => {
        {
            let mut map = $crate::MapType::default();
            $(map.insert($arg, unsafe { std::mem::transmute($obj::$name as *const ()) });)*
            $obj::method_table().lock().borrow_mut().extend(map)
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _super_init {
    ($obj:expr) => {
        {
            let o = $obj;
            let o1 = o.vtable();
            let mut o1 = o1.lock();
            o1.borrow_mut().map.extend(Self::method_table().lock().borrow_mut().clone());
            o1.borrow_mut().tids.push(std::any::TypeId::of::<Self>());
            o
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _super_call {
    ($obj:expr, $name:tt) => {
        {
            let val = $crate::OLazy::get(&$obj.method_table1()).unwrap();
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
            let val = $crate::OLazy::get($obj.method_table1()).unwrap();
            let ret = match val.lock().borrow().get(name) {
                Some(v) => {
                    Some(
                        (std::mem::transmute::<_, fn(*mut $crate::Object, $($crate::_expr_as_underscore!($arg)),+) -> _>(*v as fn(_)))(
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

#[doc(hidden)]
#[macro_export]
macro_rules! _is_child_of {
    ($e:expr, $i:ident) => {
        {
            $e.vtable().lock().borrow().tids.contains(&std::any::TypeId::of::<$i>())
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! _is_instance_of {
    ($e:expr, $i:ident) => {
        {
            *$e.vtable().lock().borrow().tids.last().unwrap() == std::any::TypeId::of::<$i>()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Object {
    vtable: crate::OVTable,
}

#[doc(hidden)]
#[allow(non_upper_case_globals)]
pub static Object_METHOD_TABLE: LazyVTable = LazyVTable::new(|| unsafe {
    let mut map = BTreeMap::new();
    map.insert("is_object", std::mem::transmute(Object::is_object as *const ()));
    map.insert("has_method:", std::mem::transmute(Object::has_method as *const ()));
    VTableInner::new(RefCell::new(map))
});

impl Default for Object {
    fn default() -> Self {
        Lazy::force(&Object_METHOD_TABLE);
        let m = Lazy::get(&Object_METHOD_TABLE).unwrap();
        let map = 
            m.lock().clone().into_inner();
        let tids = vec![std::any::TypeId::of::<Self>()];
        Self { vtable: Arc::new(ReentrantMutex::new(RefCell::new(VTable{ map, tids }))) }
    }
}

impl Object {
    pub fn vtable(&self) -> OVTable {
        return self.vtable.clone();
    }

    pub fn method_table() -> &'static LazyVTable {
        Lazy::force(&Object_METHOD_TABLE);
        &Object_METHOD_TABLE
    }

    pub fn method_table1(&self) -> &'static LazyVTable {
        Lazy::force(&Object_METHOD_TABLE);
        &Object_METHOD_TABLE
    }

    pub fn get_method(&self, name: &'static str) -> Option<fn(*mut Object)> {
        if let Some(f) = self.vtable().lock().borrow().map.get(name) {
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
        self.vtable().lock().borrow_mut().map.insert(name, std::mem::transmute(f))
    }

    pub unsafe fn try_add_method(&mut self, name: &'static str, f: *const ()) -> Result<(), &str> {
        let t = self.vtable();
        let t = t.lock();
        if t.borrow().map.contains_key(name) {
            Err("Key exists")
        } else {
            t.borrow_mut().map.insert(name, std::mem::transmute(f));
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

    pub fn has_method(&self, nm: &str) -> bool {
        self.vtable().lock().borrow().map.contains_key(nm)
    }

    pub fn is_object(&self) -> bool {
        true
    }

    pub fn print_methods(&self) {
        println!("{:?}", self.vtable().lock().borrow().map)
    }
}
