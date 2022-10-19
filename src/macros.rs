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
        #[derive(derive_builder::Builder, Default, mlua::ToLua)]
        #[builder(setter(strip_option))]
        #[builder(build_fn(private, name = "build_"))]
        #[builder(default)]
        pub struct $ident$(<$($generics),*>)? {$($body)*}
        concat_idents::concat_idents!(Builder = $ident, Builder{
            impl$(<$($generics),*>)? $ident$(<$($generics),*>)? {
                    pub fn builder() -> Builder$(<$($generics),*>)? {
                        Default::default()
                    }
            }
            impl$(<$($generics),*>)? Builder$(<$($generics),*>)? {
                    pub fn build(&self) -> $ident$(<$($generics),*>)? {
                        self.build_().expect("builder is infallible")
                    }
            }
        });

    };
    {fallible $ident:ident$(<$($generics:tt),*>)? {$($body:tt)*} } => {
        #[derive(Builder, Default, ToLua)]
        #[builder(setter(strip_option))]
        #[builder(default)]
        pub struct $ident$(<$($generics),*>)? {$($body)*}
        concat_idents::concat_idents!(Builder = $ident, Builder{
            impl$(<$($generics),*>)? $ident$(<$($generics),*>)? {
                    pub fn builder() -> Builder$(<$($generics),*>)? {
                        Default::default()
                    }
            }
        });

    };
}
