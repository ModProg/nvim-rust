use lsp::Lsp;
use mlua::prelude::*;
use nvim_macros::api;

#[macro_use]
mod macros;

pub mod api;
use api::Api;
pub mod common;
pub mod lsp;

api! {
    /// `vim.*`
    Vim("https://neovim.io/doc/user/lua.html" # "lua-stdlib"){
        api: Api,
        lsp: Lsp,
        /// Display a notification to the user
        #[p = "vim."]
        notify(value: &str, level: LogLevel, opts: Option<LuaTable>) {
            #[derive(Debug, Clone, Copy, ToLua)]
            pub enum LogLevel {
                Trace = 0,
                Debug = 1,
                Info = 2,
                Warn = 3,
                Error = 4,
            }
        }
        // g: Variables,
        // b: IndexedVariables,
        // w: IndexedVariables,
        // t: IndexedVariables,
        // v: Variables,
    }
}

impl<'lua> Vim<'lua> {
    pub fn new(lua: &'lua Lua) -> Self {
        Self::from_lua(lua.globals().get("vim").unwrap(), lua).unwrap()
    }
}

impl<'lua> From<&'lua Lua> for Vim<'lua> {
    fn from(lua: &'lua Lua) -> Self {
        Self::new(lua)
    }
}
