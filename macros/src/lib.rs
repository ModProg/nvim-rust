use std::iter;

use proc_macro2::TokenStream;
use proc_macro_error::{proc_macro_error, ResultExt};
use quote_use::quote_use as quote;
use syn::{
    braced, parenthesized,
    parse::{Parse, ParseStream, Parser},
    punctuated::Punctuated,
    token::Brace,
    Attribute, FnArg, Ident, LitStr, Pat, PatIdent, PatType, ReturnType, Token, Type,
};

macro_rules! sequence {
    ($($ty:ty),*) => {
        |input: ParseStream| -> syn::Result<_> {
            Ok(($(<$ty as Parse>::parse(input)?),*))
        }
    };
}

#[proc_macro_error]
#[proc_macro]
pub fn api(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parser = |input: ParseStream| -> syn::Result<TokenStream> {
        let mut attrs = Attribute::parse_outer(input)?;
        let struct_ident: Ident = input.parse()?;
        let mut prefix = String::new();
        attrs.retain(|attr| {
            if attr.path.is_ident("prefixed") {
                prefix = format!("{struct_ident}.").to_lowercase();
                false
            } else {
                true
            }
        });
        let inner;
        let _ = parenthesized!(inner in input);
        let base = inner.parse::<LitStr>()?.value();
        let section = sequence!(Token![#], LitStr)(&inner)
            .map(|(_, lit)| format!("#{}", lit.value()))
            .unwrap_or_default();
        let doc_link = format!("[Neovim doc]({base}{section})");

        let (empty_fields, (fields, (fns, tts))): (Vec<_>, (Vec<_>, (Vec<_>, Vec<_>))) = if input
            .peek(Brace)
        {
            let inner;
            let _ = braced!(inner in input);
            let input = inner;

            iter::from_fn(|| {
                if input.is_empty() {
                    None
                } else {
                Some((|| {
                let mut attrs = Attribute::parse_outer(&input)?;
                let ident: Ident = input.parse()?;
                let mut section = if input.peek(Token![:]) { ident.to_string() } else{ format!("{ident}%28%29") };

                let mut prefix = prefix.clone();
                attrs.retain(|attr| if matches!(attr.path.get_ident(), Some(ident) if "prefix".starts_with(&ident.to_string())) {
                    prefix = if let Ok((_, section )) = sequence!(Token![=], LitStr).parse2(attr.tokens.clone()){
                        section.value()
                    } else {
                        format!("{struct_ident}.")
                    };
                    false
                }else{true});

                section = prefix + &section;

                attrs.retain(|attr| if matches!(attr.path.get_ident(), Some(ident) if "section".starts_with(&ident.to_string())) {
                    section = sequence!(Token![=], LitStr).parse2(attr.tokens.clone()).unwrap_or_abort().1.value();
                    false
                }else{true});
                let mut doc_link = format!("{base}#{section}");
                attrs.retain(|attr| if matches!(attr.path.get_ident(), Some(ident) if "link".starts_with(&ident.to_string())) {
                    doc_link = sequence!(Token![=], LitStr).parse2(attr.tokens.clone()).unwrap_or_abort().1.value();
                    false
                }else{true});
                let doc_link = format!("\n\n[Neovim doc]({doc_link})");
                let ident_s = ident.to_string();
                Ok((quote!(#ident: ::once_cell::unsync::OnceCell::new()),
                if input.parse::<Token![:]>().is_ok() {
                    let ty: Type = input.parse()?;
                    let _  = input.parse::<Token![,]>();
                (
                    quote!{
                        #(#attrs)*
                        #[doc = #doc_link]
                        #ident: ::once_cell::unsync::OnceCell<#ty<'lua>>
                    }, (
                    quote!{
                        #(#attrs)*
                        #[doc = #doc_link]
                        pub fn #ident(&self) -> &#ty<'lua> {
                            self.#ident.get_or_init(|| {
                                self.this.get(#ident_s).unwrap()
                            })
                        }
                    },
                    quote!())
                )
                } else {
                let inner;
                let _ = parenthesized!(inner in input);
                let fields = Punctuated::<FnArg, Token![,]>::parse_terminated(&inner)?;
                let names = fields.iter().map(|arg|match arg {
                    FnArg::Typed(PatType{pat,..}) => match pat.as_ref() {
                        Pat::Ident(PatIdent{ident,..}) => ident,
                        _ => unimplemented!()
                    },
                        _ => unimplemented!()
                });
                let ret_type:ReturnType = input.parse()?;
                let tt = if input.peek(Brace) {
                    let inner;
                    _ = braced!(inner in input);
                    inner.parse()?
                } else {
                    quote!()
                };
                (quote!{
                    #(#attrs)*
                    #[doc = #doc_link]
                    #ident: ::once_cell::unsync::OnceCell<LuaFunction<'lua>>
                },(quote!{
                    #(#attrs)*
                    #[doc = #doc_link]
                    pub fn #ident(&self, #fields) #ret_type {
                        mlua::FromLuaMulti::from_lua_multi(
                            self.#ident.get_or_init(|| {
                                self.this.get(#ident_s).unwrap()
                            })
                            .call((#(#names),*)).unwrap(), self.lua).unwrap()
                    }
                },tt))}
                ))})().unwrap_or_abort())}
            }).unzip()
        } else {
            Default::default()
        };

        Ok(quote! {
            # use mlua::prelude::{LuaTable, LuaResult, LuaValue};
            # use mlua::{FromLua, Lua};
            #(#attrs)*
            #[doc = #doc_link]
            pub struct #struct_ident<'lua> {
                lua: &'lua Lua,
                this: LuaTable<'lua>,
                #(#fields),*
            }
            impl<'lua> #struct_ident<'lua> {
                #(#fns)*
            }
            impl<'lua> FromLua<'lua> for #struct_ident<'lua>{
                fn from_lua(lua_value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
                    Ok(Self{
                        lua,
                        this: FromLua::from_lua(lua_value, lua)?,
                        #(#empty_fields),*
                    })
                }
            }
            #(#tts)*
        })
    };
    |input: ParseStream| -> syn::Result<TokenStream> {
        iter::from_fn(|| {
            if input.is_empty() {
                None
            } else {
                Some(parser(input))
            }
        })
        .collect()
    }
    .parse(input)
    .unwrap_or_abort()
    .into()
}
