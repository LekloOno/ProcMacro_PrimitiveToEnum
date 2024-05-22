use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

#[proc_macro_derive(FromU32)]
pub fn from_u32_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_from_u32_macro(&ast)
}

fn impl_from_u32_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let Data::Enum(data_enum) = &ast.data else { todo!()};
    let variants = &data_enum.variants;
    let cases: Vec<_> = variants
        .iter()
        .map(|variant| {
            let discriminant = &variant.discriminant.as_ref().unwrap();
            let discriminant_expr = &discriminant.1;
            let variant_name = &variant.ident;
            quote! {
                #discriminant_expr => ::core::result::Result::Ok(#name::#variant_name),
            }
        })
        .collect();

    let from_impl = quote! {
        impl #name {
            pub fn from_u32(n: u32) -> ::core::result::Result<#name, ()> {
                match n {
                    #(#cases)*
                    _ => ::core::result::Result::Err(()),
                }
            }
        }
    };

    //panic!("{}", from_impl);
    from_impl.into()
}