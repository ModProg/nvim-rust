use mlua::prelude::*;
use nvim_macros::fn_table;

#[macro_use]
mod macros;

pub mod api;
pub mod common;
pub mod lsp;

fn_table! {
    /// `vim.*`
    Vim(vim, "https://neovim.io/doc/user/lua.html" # "lua-stdlib") {
        /// Display a notification to the user
        [prefixed] notify(value: &str, level: LogLevel, opts: Option<LuaTable>) {
            #[derive(Debug, Clone, Copy, ToLua)]
            pub enum LogLevel {
                Trace = 0,
                Debug = 1,
                Info = 2,
                Warn = 3,
                Error = 4,
            }
        }
    }
}
