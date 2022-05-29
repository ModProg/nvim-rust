use std::collections::HashMap;

use mlua::prelude::*;
use nvim_macros::fn_table;

use crate::common::Buffer;

fn_table! {
    /// `vim.lsp.*`
    Lsp(vim.lsp, "https://neovim.io/doc/user/lsp.html"#"LSP") {
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
