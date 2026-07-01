mod class;
mod dts;
mod function;
mod module;
mod types;

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn, ItemImpl, ItemMod, ItemStruct};

use class::expand_class_struct;
use function::expand_function;
use module::expand_module;

#[proc_macro_attribute]
pub fn tails_function(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_fn = parse_macro_input!(item as ItemFn);
    let options = function::parse_fn_options(&item_fn.attrs);
    expand_function(item_fn, options).into()
}

#[proc_macro_attribute]
pub fn tails_module(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_mod = parse_macro_input!(item as ItemMod);
    let mut options = module::parse_module_options(&item_mod.attrs);

    if !attr.is_empty() {
        let attr_tokens: proc_macro2::TokenStream = attr.into();
        let attr_str = attr_tokens.to_string();
        if attr_str.contains("name") {
            if let Some(start) = attr_str.find("name") {
                let rest = &attr_str[start..];
                if let Some(eq_pos) = rest.find('=') {
                    let after_eq = rest[eq_pos + 1..].trim();
                    if after_eq.starts_with('"') && after_eq.ends_with('"') {
                        options.name = Some(after_eq[1..after_eq.len() - 1].to_string());
                    }
                }
            }
        }
    }

    expand_module(item_mod, options).into()
}

#[proc_macro_attribute]
pub fn tails_class(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_impl = parse_macro_input!(item as ItemImpl);
    expand_class_struct(item_impl).into()
}
