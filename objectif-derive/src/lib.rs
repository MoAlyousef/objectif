extern crate proc_macro;
use proc_macro::TokenStream;
// use quote::quote;

#[proc_macro]
pub fn call_method(input: TokenStream) -> TokenStream {
    format!("objectif_core::call_method![{}]", input)
        .parse()
        .unwrap()
}

#[proc_macro]
pub fn add_class_method(input: TokenStream) -> TokenStream {
    format!("objectif_core::add_class_method![{}]", input)
        .parse()
        .unwrap()
}

#[proc_macro]
pub fn define_class(input: TokenStream) -> TokenStream {
    format!("objectif_core::define_class![{}]", input)
        .parse()
        .unwrap()
}

#[proc_macro_attribute]
pub fn inherits(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input1 = input.to_string();
    let input1: Vec<_> = input1.split(' ').collect();
    let class_name = input1[1];
    format!(
        "{input}\n
        objectif_core::define_class![{class_name}:{attr}];\n
        #[allow(non_upper_case_globals)]\n
        static {class_name}_METHOD_TABLE: objectif_core::LazyVTable = objectif_core::LazyVTable::new(|| objectif_core::VTableInner::new(objectif_core::RCellMapType::new(objectif_core::MapType::default())));\n
        impl {class_name} {{
            pub fn method_table() -> &'static objectif_core::LazyVTable {{
                &{class_name}_METHOD_TABLE
            }}
        }}
        "
    )
    .parse()
    .unwrap()
}
