use std::collections::HashMap;

use mlua::prelude::*;
use nvim_macros::api;

use crate::common::Buffer;

api! {
    /// `vim.lsp.*`
    Lsp("https://neovim.io/doc/user/lsp.html"#"LSP") {
        /// Sends an async request for all active clients attached to the buffer.
        buf_request(buffer: Buffer, method: String, params: Option<LuaTable<'lua>>, handler: Handler<'lua>) -> (Option<HashMap<String, String>>, LuaFunction<'lua>) {

            #[derive(ToLua)]
            pub enum Handler<'lua> {
                #[mlua(value = lua.create_function(|_,()| Ok(LuaValue::Nil)).unwrap())]
                None,
                #[mlua(value = LuaValue::Nil)]
                Default,
                LuaFunction(LuaFunction<'lua>)
            }
        }
    }
}
