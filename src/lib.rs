extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{Body, VariantData};

#[proc_macro_derive(__objc_methname, attributes(value))]
pub fn derive_objc_methname(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let ast = syn::parse_macro_input(&source).unwrap();

    // Get the value, it will either be of the form of a single rust identifier,
    // or a series of rust identifiers ended by a :. The first case will be
    // handled by the struct name, while the second will be handled by a struct
    // with dummy fields, each of which is one of the rust identifiers. It's a
    // hack.
    let mut value = match ast.body {
        Body::Struct(VariantData::Unit) =>
            ast.ident.to_string().into_bytes(),
        Body::Struct(VariantData::Struct(ref fields)) => {
            let mut result = Vec::new();
            for field in fields {
                result.extend(field.ident.as_ref().unwrap().as_ref().as_bytes());
                result.push(b':');
            }
            result
        }
        _ => panic!("Unexpected struct format as argument to derive(__objc_methname)")
    };
    value.push(b'\0'); // Add the null to the end of the string

    let length = value.len();
    let name = &ast.ident;
    let result = quote!{
        impl #name {
            #[inline]
            fn get() -> *const u8 {
                #[repr(C)]
                struct SendSyncWrap(*const [u8; #length]);
                unsafe impl Send for SendSyncWrap {}
                unsafe impl Sync for SendSyncWrap {}

                #[link_section="__TEXT,__objc_methname,cstring_literals"]
                static VALUE : [u8; #length] = #value;
                #[link_section="__DATA,__objc_selrefs,literal_pointers,no_dead_strip"]
                static REF : SendSyncWrap = SendSyncWrap(&VALUE);

                REF.0 as *const u8
            }
        }
    };

    result.to_string().parse().unwrap()
}

