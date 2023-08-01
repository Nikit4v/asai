extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Spacing;
use proc_macro2::TokenTree::Punct;
use quote::__private::ext::RepToTokensExt;
use quote::ToTokens;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Meta, Expr};
use syn::spanned::Spanned;

#[proc_macro_derive(FromLine, attributes(name, default))]
pub fn derive_from_line(_item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(_item as DeriveInput);
    let mut names: Vec<proc_macro2::TokenStream> = vec![];
    let mut field_names: Vec<proc_macro2::TokenStream> = vec![];
    let mut field_types: Vec<proc_macro2::TokenStream> = vec![];
    let mut defaults: Vec<proc_macro2::TokenStream> = vec![];

    if let Data::Struct(d) = input.data {
        let type_name: proc_macro2::TokenStream = input.ident.into_token_stream().into();
        if let Fields::Named(fields) = d.fields {
            for field in fields.named {
                let mut name: Option<_> = None;
                let mut default_expr = (quote! {Default::default()}).into_token_stream();
                let mut default_set = false;

                for attr in field.attrs {
                    if attr.path().get_ident().unwrap().to_string() == "name" {
                        let expr = &attr.meta.require_list().unwrap().tokens.next().unwrap();
                        name = Some(expr.into_token_stream())
                    } else if attr.path().get_ident().unwrap().to_string() == "default" {
                        let expr = &attr.meta.require_list().unwrap().tokens.next().unwrap();
                        default_expr = expr.into_token_stream();
                        default_expr = (quote! {Some(#default_expr)}).into_token_stream();
                        default_set = true;
                    }
                }
                let ty = field.ty.into_token_stream();
                names.push(name.unwrap());
                field_names.push(field.ident.unwrap().into_token_stream());
                field_types.push(quote!{ Option<#ty> }.into_token_stream());
                defaults.push(default_expr)
            }
        }
        (quote! {
            impl<'a> asai::structure::FromLine<'a> for #type_name<'a> {
                fn from_line(line: &'a str, format: &str) -> Result<Self, asai::structure::InvalidValue> {
                    #(let mut #field_names: #field_types = #defaults;)*

                    let format_fields: Vec<_>  = format.split(',').map(str::trim).collect();
                    let line_fields  = line.splitn(format_fields.len(), ',');
                    for (k, v) in format_fields.into_iter().zip(line_fields) {
                        match k {
                            #(#names => #field_names = Some(asai::structure::base_types::LineField::new(v).try_into()?), )*

                            _ => ()
                        }
                    }
                    Ok(Self {
                        #(#field_names: #field_names.ok_or(asai::structure::InvalidValue)?, )*
                    })
                }
            }
        }).into()
    } else {
        panic!()
    }
}
