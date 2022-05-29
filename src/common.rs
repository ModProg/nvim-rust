use derive_more::From;
use mlua::ToLua;

#[derive(Clone, ToLua, From)]
pub enum Buffer {
    #[mlua(value = 0)]
    Current,
    Index(i64),
}
