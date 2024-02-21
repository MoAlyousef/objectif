extern crate proc_macro;
use proc_macro::TokenStream;
// use quote::quote;

#[proc_macro]
pub fn call_method(input: TokenStream) -> TokenStream {
    format!("objectif::_call_method![{input}]")
        .parse()
        .unwrap()
}

#[proc_macro]
pub fn add_class_method(input: TokenStream) -> TokenStream {
    format!("objectif::_add_class_method![{input}]")
        .parse()
        .unwrap()
}

#[proc_macro]
pub fn super_init(input: TokenStream) -> TokenStream {
    format!("objectif::_super_init![{}]", input)
        .parse()
        .unwrap()
}

#[proc_macro_attribute]
pub fn inherits(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input1 = input.to_string();
    let input1: Vec<_> = input1.split(' ').collect();
    let class_name = input1[1];
    let fmt = format!(
        "{input}\n
        objectif::_define_class![{class_name}:{attr}];\n
        #[allow(non_upper_case_globals)]\n
        static {class_name}_METHOD_TABLE: objectif::LazyVTable = objectif::LazyVTable::new(|| objectif::VTableInner::new(objectif::RCellMapType::new(objectif::Lazy::get(&{attr}::method_table()).expect(\"oops\").lock().clone().into_inner())));\n
        impl {class_name} {{
            pub fn method_table() -> &'static objectif::LazyVTable {{
                objectif::Lazy::force(&{class_name}_METHOD_TABLE);
                &{class_name}_METHOD_TABLE
            }}
            pub fn method_table1(&self) -> &'static objectif::LazyVTable {{
                objectif::Lazy::force(&{class_name}_METHOD_TABLE);
                &{class_name}_METHOD_TABLE
            }}
        }}
        "
    );
    fmt.parse()
    .unwrap()
}

#[proc_macro]
pub fn table_init(input: TokenStream) -> TokenStream {
    let fmt = format!("objectif::_table_init!({input})");
    fmt.parse().unwrap()
}

#[proc_macro]
pub fn super_call(input: TokenStream) -> TokenStream {
    let fmt = format!("objectif::_super_call!({input})");
    fmt.parse().unwrap()
}