use common::Index;
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
    #[prefixed]
    Vim("https://neovim.io/doc/user/lua.html" # "lua-stdlib"){
        api: Api,
        #[link="https://neovim.io/doc/user/lsp.html#LSP"]
        lsp: Lsp,
        /// Display a notification to the user
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
        g: Variables,
        b: IndexedVariables,
        w: IndexedVariables,
        t: IndexedVariables,
        v: Variables,
        env: Env,
    }
}

impl<'lua> Vim<'lua> {
    pub fn new(lua: &'lua Lua) -> Self {
        Self::from_lua(lua.globals().get("vim").unwrap(), lua).unwrap()
    }

    pub fn lua(&self) -> &'lua Lua {
        self.lua
    }
}

impl<'lua> From<&'lua Lua> for Vim<'lua> {
    fn from(lua: &'lua Lua) -> Self {
        Self::new(lua)
    }
}

api! {
    Variables("https://neovim.io/doc/user/lua.html"#"lua-vim-variables")
    IndexedVariables("https://neovim.io/doc/user/lua.html"#"lua-vim-variables")
    Env("https://neovim.io/doc/user/lua.html"#"vim.env")
}

impl<'lua> Variables<'lua> {
    pub fn get<V: FromLua<'lua>>(&self, name: impl AsRef<str>) -> LuaResult<V> {
        self.this.get(name.as_ref())
    }
    pub fn set(&self, name: impl AsRef<str>, value: impl ToLua<'lua>) {
        self.this.set(name.as_ref(), value).unwrap()
    }
    pub fn unset(&self, name: impl AsRef<str>) {
        self.this.set(name.as_ref(), LuaValue::Nil).unwrap()
    }
}

impl<'lua> IndexedVariables<'lua> {
    pub fn get<V: FromLua<'lua>>(&self, index: Index, name: impl AsRef<str>) -> LuaResult<V> {
        if let Index::Index(index) = index {
            self.this.get::<_, LuaTable>(index)?.get(name.as_ref())
        } else {
            self.this.get(name.as_ref())
        }
    }
    pub fn set(
        &self,
        index: Index,
        name: impl AsRef<str>,
        value: impl ToLua<'lua>,
    ) -> LuaResult<()> {
        if let Index::Index(index) = index {
            self.this
                .get::<_, LuaTable>(index)?
                .set(name.as_ref(), value)
                .unwrap()
        } else {
            self.this.set(name.as_ref(), value).unwrap()
        }
        Ok(())
    }
    pub fn unset(&self, index: Index, name: impl AsRef<str>) -> LuaResult<()> {
        if let Index::Index(index) = index {
            self.this
                .get::<_, LuaTable>(index)?
                .set(name.as_ref(), LuaValue::Nil)
                .unwrap()
        } else {
            self.this.set(name.as_ref(), LuaValue::Nil).unwrap()
        }
        Ok(())
    }
}

impl Env<'_> {
    pub fn get(&self, name: impl AsRef<str>) -> Option<String> {
        self.this.get(name.as_ref()).unwrap()
    }
    pub fn set(&self, name: impl AsRef<str>, value: impl AsRef<str>) {
        self.this.set(name.as_ref(), value.as_ref()).unwrap()
    }
    pub fn unset(&self, name: impl AsRef<str>) {
        self.this.set(name.as_ref(), LuaValue::Nil).unwrap()
    }
}
