use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemMod, LitStr, Meta};

pub struct ModuleOptions {
    pub name: Option<String>,
}

pub fn parse_module_options(attrs: &[syn::Attribute]) -> ModuleOptions {
    let mut name = None;
    for attr in attrs {
        if attr.path().is_ident("tails") {
            if let Meta::List(list) = &attr.meta {
                if let Ok(nested) = &list.parse_nested_meta(|meta| {
                    if meta.path.is_ident("name") {
                        let value = meta.value()?;
                        let lit: LitStr = value.parse()?;
                        name = Some(lit.value());
                        Ok(())
                    } else {
                        Err(meta.error("unknown tails attribute"))
                    }
                }) {
                    let _ = nested;
                }
            }
        }
    }
    ModuleOptions { name }
}

pub fn expand_module(item: ItemMod, options: ModuleOptions) -> TokenStream {
    let vis = &item.vis;
    let mod_name = &item.ident;
    let content = item.content.as_ref().map(|(_, items)| items);
    let attrs = &item.attrs;

    let module_name = options.name.unwrap_or_else(|| {
        mod_name.to_string().replace('_', "-").to_lowercase()
    });

    // Find all functions in the module and extract their FFI names
    let mut registrations = Vec::new();
    let mut function_items = Vec::new();

    if let Some(items) = content {
        for item in items {
            match item {
                syn::Item::Fn(func) => {
                    let func_name_str = func.sig.ident.to_string();

                    // Skip macro-generated helpers (FFI wrappers, class registration, etc.)
                    if func_name_str.starts_with("__tails_") {
                        function_items.push(quote! { #func });
                        continue;
                    }

                    // Check for #[tails(js_name = "...")] attribute
                    let actual_js_name = extract_js_name(&func.attrs)
                        .unwrap_or_else(|| func_name_str.clone());

                    let ffi_name = format_ident!("__tails_ffi_{}", func.sig.ident);
                    registrations.push(quote! {
                        handle.module.register(#actual_js_name, #ffi_name as ::tails_abi::NativeFn);
                    });

                    function_items.push(quote! { #func });
                }
                syn::Item::Struct(s) => {
                    let struct_name = &s.ident;
                    let regs_name = format_ident!("__TAILS_CLASS_REGS_{}", struct_name);

                    // Drain class method registrations
                    registrations.push(quote! {
                        {
                            let mut regs = #regs_name.lock().unwrap();
                            for (name, func) in regs.drain(..) {
                                handle.module.register(name, func);
                            }
                        }
                    });

                    function_items.push(quote! { #s });
                }
                other => {
                    function_items.push(quote! { #other });
                }
            }
        }
    }

    let init_fn_name = format_ident!("tails_native_init");
    let meta_name = format_ident!(
        "__TAILS_MODULE_META_{}",
        module_name.replace('-', "_").to_uppercase()
    );

    let meta_fn = quote! {
        #[used]
        #[doc(hidden)]
        #[no_mangle]
        pub static #meta_name: &str = #module_name;
    };

    let init_fn = quote! {
        #[no_mangle]
        pub extern "C" fn #init_fn_name() -> *mut ::tails_abi::ModuleHandle {
            let module = ::tails_abi::NativeModule::new(#module_name);
            let mut handle = ::tails_abi::ModuleHandle::new(module);

            #(#registrations)*

            Box::into_raw(Box::new(handle))
        }
    };

    quote! {
        #(#attrs)*
        #vis mod #mod_name {
            #(#function_items)*

            #init_fn
            #meta_fn
        }
    }
}

fn extract_js_name(attrs: &[syn::Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident("tails") {
            if let Meta::List(list) = &attr.meta {
                let mut js_name = None;
                let _ = list.parse_nested_meta(|meta| {
                    if meta.path.is_ident("js_name") {
                        let value = meta.value()?;
                        let lit: LitStr = value.parse()?;
                        js_name = Some(lit.value());
                        Ok(())
                    } else {
                        Ok(())
                    }
                });
                if js_name.is_some() {
                    return js_name;
                }
            }
        }
    }
    None
}