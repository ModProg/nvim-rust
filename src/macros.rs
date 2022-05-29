// macro_rules! fn_table {
//     {
//         $(#[doc = $struct_doc:expr])*
//         $struct:ident($global:ident$(.$($path:ident).*)?, $base:literal $(# $sec:literal)?){
//         $(
//             $(#[doc = $doc:expr])*
//             $([no_prefix])? $ident:ident($($param:ident : $param_type:ty),*) $(-> $ret_type:ty)? $({
//                 $($tts:tt)*
//             })?
//         )*
//     }} => {
//         $(#[doc = $struct_doc])*
//         #[doc = concat!("[Neovim doc](", $base, "#", $($sec,)? ")")]
//         #[derive(FromLua)]
//         pub struct $struct<'lua> {
//             #[mlua(lua)]
//             lua: &'lua Lua,
//             $(
//                 $(#[doc = $doc])*
//                 #[doc = concat!("\n\n[Neovim doc](", $base, "#", $($prefix, ".",)? stringify!($ident), "%28%29)")]
//                 $ident: LuaFunction<'lua>,
//             )*
//         }
//         impl<'lua> From<&'lua Lua> for $struct<'lua>{
//             fn from(lua: &'lua Lua) -> Self {
//                 let value: LuaValue = lua.globals().get(stringify!($global)).unwrap();
//                 $($(let value: LuaValue = LuaTable::from_lua(value, lua).unwrap().get(stringify!($path)).unwrap();)*)?
//                 Self::from_lua(value, lua).unwrap()
//             }
//         }
//         impl<'lua> $struct<'lua> {
//             $(
//                 $(#[doc = $doc])*
//                 #[doc = concat!("\n\n[Neovim doc](", $base, "#", $($prefix, ".",)? stringify!($ident), "%28%29)")]
//                 pub fn $ident(&self, $($param: $param_type),*) $(-> $ret_type)? {
//                     FromLuaMulti::from_lua_multi(self.$ident.call(($($param),*)).expect("a"), self.lua).expect("b")
//                 }
//             )*
//         }
//         $($($($tts)*)*)*
//     };
// }

macro_rules! input {
    {$ident:ident$(<$($generics:tt),*>)? {$($body:tt)*} } => {
        #[derive(Builder, Default, ToLua)]
        #[builder(setter(strip_option))]
        #[builder(default)]
        pub struct $ident$(<$($generics),*>)? {$($body)*}
        impl$(<$($generics),*>)? $ident$(<$($generics),*>)? {
            concat_idents::concat_idents!(Builder = $ident, Builder{
                pub fn builder() -> Builder$(<$($generics),*>)? {
                    Default::default()
                }
            });
        }
    };
}

// macro_rules! builder_opt {
//     {$struct:ident {
//         $($ident:ident : $ty:ty),*
//     }} => {
//         #[skip_serializing_none]
//         #[derive(Serialize)]
//         pub struct $struct {
//             $($ident: Option<$ty>),*
//         }
//
//         impl $struct {
//             fn none()->Self{
//                 Self {
//                     $($ident: None),*
//                 }
//             }
//             $(field!{$ident: Option<$ty>})*
//         }
//     };
// }
//
// macro_rules! field {
//     ($ident:ident : Option<Vec<$ty:ty>>) => {
//         pub fn $ident<T: Into<$ty>>($ident: T) -> Self {
//             Self {
//                 $ident: Some(vec![$ident.into()]),
//                 ..Self::none()
//             }
//         }
//         pub fn $ident<T: IntoIter<Item = $ty>>($ident: T) -> Self {
//             Self {
//                 $ident: Some(vec![$ident.into_iter().collect()]),
//                 ..Self::none()
//             }
//         }
//     };
//     ($ident:ident : Option<$ty:ty>) => {
//         pub fn $ident<T: Into<$ty>>($ident: T) -> Self {
//             Self {
//                 $ident: Some($ident.into()),
//                 ..Self::none()
//             }
//         }
//     };
// }

// group: Option<StrU32>,
// event: Option<Vec<String>>,
// pattern: Option<Vec<String>>,
