use std::iter;

use proc_macro2::TokenStream;
use proc_macro_error::{ResultExt, proc_macro_error};
use syn::{
    braced, custom_keyword,
    parse::Parse,
    parse::{ParseStream, Parser},
    punctuated::Punctuated,
     Ident, Token, Attribute, parenthesized, LitStr,  token::{Bracket, Brace}, bracketed, FnArg, ReturnType, PatType, PatIdent, Pat, Type,
};

use quote_use::quote_use as quote;

macro_rules! sequence {
    ($($ty:ty),*) => {
        |input: ParseStream| -> syn::Result<_> {
            Ok(($(<$ty as Parse>::parse(input)?),*))
        }
    };
}

// #[proc_macro]
// pub fn builder(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     inner.parse(input).unwrap_or_abort().into()
// }
//
// fn inner(input: ParseStream) -> syn::Result<TokenStream> {
//     let ident: Ident = input.parse()?;
//     let _generics = Generics::parse(input).ok();
//     let content;
//     braced!(content in input);
//     let fields = Punctuated::<_, Token![,]>::parse_terminated_with(
//         &content,
//         sequence!(Ident, Token![:], FieldType),
//     )?;
//
//     let (definition, fns): (Vec<_>, Vec<_>) = fields
//         .iter()
//         .map(|(ident, _, ty)| {
//             let fields: Vec<_> = fields
//                 .iter()
//                 .filter_map(|(field, ..)| {
//                     (field != ident).then(|| quote!(#field: core::default::Default::default()))
//                 })
//                 .collect();
//             let with_ident = format_ident!("with_{ident}");
//             match ty {
//                 FieldType::Vec(outer, inner) => {
//                     let idents = format_ident!("{ident}s");
//                     let with_idents = format_ident!("with_{idents}");
//                     (
//                         quote!(#ident: Option<#outer>),
//                         quote! {
//                             pub fn #ident<T: Into<#inner>>(#ident: T) -> Self {
//                                 Self {
//                                     #ident: Some(vec![#ident.into()]),
//                                     #(#fields),*
//                                 }
//                             }
//                             pub fn #with_ident<T: Into<#inner>>(mut self, #ident: T) -> Self {
//                                 self.#ident= Some(vec![#ident.into()]);
//                                 self
//                             }
//                             pub fn #idents<T: IntoIterator<Item = #inner>>(#idents: T) -> Self {
//                                 Self {
//                                     #ident: Some(#idents.into_iter().collect()),
//                                     #(#fields),*
//                                 }
//                             }
//                             pub fn #with_idents<T: IntoIterator<Item = #inner>>(mut self, #ident: T) -> Self {
//                                 self.#ident = Some(#ident.into_iter().collect());
//                                 self
//                             }
//                         },
//                     )
//                 }
//                 FieldType::Ty(ty) => (
//                     quote!(#ident: Option<#ty>),
//                     quote! {
//                         pub fn #ident<T: Into<#ty>>(#ident: #ty) -> Self {
//                             Self {
//                                 #ident: Some(#ident.into()),
//                                 #(#fields),*
//                             }
//                         }
//                         pub fn #with_ident<T: Into<#ty>>(mut self, #ident: T) -> Self {
//                             self.#ident = Some(#ident.into());
//                             self
//                         }
//                     },
//                 ),
//             }
//         })
//         .unzip();
//
//     Ok(quote! {
//         #[skip_serializing_none]
//         #[derive(Serialize)]
//         pub struct #ident {
//             #(#definition),*
//         }
//
//         impl #ident {
//             #(#fns)*
//         }
//     })
// }
//
// enum FieldType {
//     Vec(TokenStream, Type),
//     Ty(Type),
// }
//
// impl Parse for FieldType {
//     fn parse(input: ParseStream) -> syn::Result<Self> {
//         custom_keyword!(Vec);
//         if let Ok((vec, lt, ty, gt)) = sequence!(Vec, Token![<], Type, Token![>])(input) {
//             Ok(Self::Vec(quote!(#vec #lt #ty #gt), ty))
//         } else {
//             Ok(Self::Ty(input.parse()?))
//         }
//     }
// }

#[proc_macro_error]
#[proc_macro]
pub fn api(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    |input: ParseStream| -> syn::Result<TokenStream> { 
        let attrs = Attribute::parse_outer(input)?;
        let ident: Ident = input.parse()?; 
        let inner;
        let _ = parenthesized!(inner in input);
        let base =  inner.parse::<LitStr>()?.value();
        let section = sequence!(Token![#], LitStr)(&inner).map(|(_, lit)|format!("#{}", lit.value())).unwrap_or_default();
        let doc_link = format!("[Neovim doc]({base}{section})");

        let inner;
        let _ = braced!(inner in input);
        let input = inner;

        let (empty_fields, (fields, (fns, tts))): (Vec<_>, (Vec<_>, (Vec<_>, Vec<_>))) = iter::from_fn(|| {
            if input.is_empty() {
                None
            } else {
            Some((|| {
            let mut attrs = Attribute::parse_outer(&input)?;
            let ident: Ident = input.parse()?;
            let mut section = format!("{ident}%28%29");
            attrs.retain(|attr| if matches!(attr.path.get_ident(), Some(ident) if "prefix".starts_with(&ident.to_string())) {
                section = sequence!(Token![=], LitStr).parse2(attr.tokens.clone()).unwrap_or_abort().1.value() + &section;
                false
            }else{true});
            attrs.retain(|attr| if matches!(attr.path.get_ident(), Some(ident) if "section".starts_with(&ident.to_string())) {
                section = sequence!(Token![=], LitStr).parse2(attr.tokens.clone()).unwrap_or_abort().1.value();
                false
            }else{true});
            let doc_link = format!("\n\n[Neovim doc]({base}#{section})");
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
        }).unzip();

        Ok(quote!{
            # use mlua::prelude::{LuaTable, LuaResult, LuaValue};
            # use mlua::{FromLua, Lua};
            #(#attrs)*
            #[doc = #doc_link]
            pub struct #ident<'lua> {
                lua: &'lua Lua,
                this: LuaTable<'lua>,
                #(#fields),*
            }
            impl<'lua> #ident<'lua> {
                #(#fns)*
            }
            impl<'lua> FromLua<'lua> for #ident<'lua>{
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
    }.parse(input).unwrap_or_abort().into()
}

#[proc_macro_error]
#[proc_macro]
pub fn fn_table(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    |input: ParseStream| -> syn::Result<TokenStream> { 
        let attrs = Attribute::parse_outer(input)?;
        let ident: Ident = input.parse()?; 
        let inner;
        let _ = parenthesized!(inner in input);
        let global: Ident = inner.parse()?;
        let path =if inner.parse::<Token![.]>().is_ok()  {
         Punctuated::<Ident, Token![.]>::parse_separated_nonempty(&inner)?.into_iter().collect()
        }else{Vec::new()};
        let _: Token![,] = inner.parse()?;
        let base =  inner.parse::<LitStr>()?.value();
        let section = sequence!(Token![#], LitStr)(&inner).map(|(_, lit)|format!("#{}", lit.value())).unwrap_or_default();
        let doc_link = format!("[Neovim doc]({base}{section})");

        let inner;
        let _ = braced!(inner in input);
        let input = inner;

        let (fields, (fns, tts)): (Vec<_>, (Vec<_>, Vec<_>)) = iter::from_fn(|| {
            if input.is_empty() {
                None
            } else {
            Some((|| {
            let attrs = Attribute::parse_outer(&input)?;
            let section = if input.peek(Bracket){
                let inner;
                let _ = bracketed!(inner in input);
                custom_keyword!(prefix);
                custom_keyword!(prefixed);
                if inner.peek(prefix) {
                    Some(sequence!(prefix, Token![=], LitStr)(&inner)?.2.value())
                } else if inner.parse::<prefixed>().is_ok() {
                    Some(format!("{global}.{}", path.iter().map(|s|format!("{s}.")).collect::<String>()))
                } else {
                    None
                }
            } else {None}.unwrap_or_default();
            let ident: Ident = input.parse()?;
            let doc_link = format!("\n\n[Neovim doc]({base}#{section}{ident}%28%29)");
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
            Ok((quote!{
                #(#attrs)*
                #[doc = #doc_link]
                #ident: LuaFunction<'lua>
            },(quote!{
                #(#attrs)*
                #[doc = #doc_link]
                pub fn #ident(&self, #fields) #ret_type {
                    mlua::FromLuaMulti::from_lua_multi(self.#ident.call((#(#names),*)).unwrap(), self.lua).unwrap()
                }
            },tt)))
            })().unwrap_or_abort())}
        }).unzip();

        Ok(quote!{
            #(#attrs)*
            #[doc = #doc_link]
            #[derive(FromLua)]
            pub struct #ident<'lua> {
                #[mlua(lua)]
                lua: &'lua Lua,
                #(#fields),*
            }
            impl<'lua> From<&'lua Lua> for #ident<'lua>{
                fn from(lua: &'lua Lua) -> Self {
                    let value: LuaValue = lua.globals().get(stringify!(#global)).unwrap();
                    #(let value: LuaValue = LuaTable::from_lua(value, lua).unwrap().get(stringify!(#path)).unwrap();)*
                    Self::from_lua(value, lua).unwrap()
                }
            }
            impl<'lua> #ident<'lua> {
                #(#fns)*
            }
            #(#tts)*
        })
    }.parse(input).unwrap_or_abort().into()
}
