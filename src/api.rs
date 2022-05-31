#![allow(non_snake_case)]
use std::future::Future;

use derive_builder::Builder;
use derive_more::From;
use futures::FutureExt;
use mlua::{chunk, prelude::*, ToLua};
use nvim_macros::api;
use serde::Deserialize;
use smart_default::SmartDefault;

use crate::common::Index;

api! {
    /// `vim.api.*`
    Api("https://neovim.io/doc/user/api.html" # "api-global") {

        nvim__get_runtime(pat: &[&str], all: bool, opts: NvimGetRuntimeOpts) -> LuaValue {
            input!{
                NvimGetRuntimeOpts {
                    is_lua: bool
                }
            }
        }

        nvim_buf_create_user_command(buffer: Index, name: String, command: Command, opts: NvimCreateUserCommandOpts) {
            input!{
                NvimCreateUserCommandOpts {
                    pub nargs: Nargs,
                    pub desc: Option<String>,
                    pub force: Option<bool>,
                }
            }

            #[derive(Clone, SmartDefault, ToLua)]
            pub enum Nargs {
                #[default]
                #[mlua(value = 0)]
                None,
                #[mlua(value = 1)]
                One,
                #[mlua(value = "*")]
                Any,
                #[mlua(value = "?")]
                NoneOrOne,
                #[mlua(value = "+")]
                OneOrMore
            }

            #[derive(ToLua, From, Clone)]
            pub enum Command<'lua> {
                VimScript(String),
                Lua(LuaFunction<'lua>),
            }


            #[derive(Clone, FromLua, Debug, ToLua)]
            pub struct CommandCallbackData {
                /// The args passed to the command [`<args>`](https://neovim.io/doc/user/map.html#%3Cargs%3E)
                pub args: Option<String>,
                /// The args split by unescaped whitespace (when more than one argument is allowed)
                /// [`<fargs>`](https://neovim.io/doc/user/map.html#%3Cf-args%3E)
                pub fargs: Vec<String>,
                /// "true" if the command was executed with a ! modifier [`<bang>`](https://neovim.io/doc/user/map.html#%3Cbang%3E)
                pub bang: bool,
                /// The starting line of the command range [`<line1>`](https://neovim.io/doc/user/map.html#%3Cline1%3E)
                pub line1: Option<i64>,
                /// The final line of the command range [`<line2>`](https://neovim.io/doc/user/map.html#%3Cline2%3E)
                pub line2: Option<i64>,
                /// The number of items in the command range: 0, 1, or 2
                /// [`<range>`](https://neovim.io/doc/user/map.html#%3Crange%3E)
                pub range: Option<i64>,
                /// Any count supplied [`<count>`](https://neovim.io/doc/user/map.html#%3Ccount%3E)
                pub count: Option<i64>,
                /// The optional register, if specified [`<reg>`](https://neovim.io/doc/user/map.html#%3Creg%3E)
                pub reg: Option<String>,
                /// Command modifiers, if any [`<mods>`](https://neovim.io/doc/user/map.html#%3Cmods%3E)
                pub mods: Option<String>
            }
        }

        /// Create an autocommand
        ///
        /// The API allows for two (mutually exclusive) types of actions
        /// to be executed when the autocommand triggers: a callback
        /// function (Lua or Vimscript), or a command (like regular
        /// autocommands).
        nvim_create_autocmd(events: Vec<String>, opts: NvimCreateAutocmdsOpts) -> i64 {
            input!{
                NvimCreateAutocmdsOpts<'lua> {
                    group: Option<StrI64>,
                    /// Cannot be used with buffer
                    pattern: Option<Vec<String>>,
                    /// Cannot be used with pattern
                    buffer: Option<i64>,
                    desc: Option<String>,
                    /// Cannot be used with command
                    callback: Option<Callback<'lua>>,
                    /// Cannot be used with callback
                    command: Option<String>,
                    once: bool,
                    nested: bool
                }
            }

            #[derive(ToLua, From, Clone)]
            pub enum Callback<'lua> {
                VimScript(String),
                Lua(LuaFunction<'lua>),
            }

            impl<'lua> Callback<'lua> {
                pub fn from_fn(lua: &'lua Lua, callback: impl Fn(&'lua Lua, AutocmdCallbackData) -> bool + 'static + Send) -> Self {
                    Callback::Lua(lua.create_function(move |lua, a| Ok(callback(lua, a))).unwrap())
                }
                pub fn from_fn_mut(lua: &'lua Lua, mut callback: impl FnMut(AutocmdCallbackData) -> bool + 'static + Send) -> Self {
                    Callback::Lua(lua.create_function_mut(move |_lua, a| Ok(callback(a))).unwrap())
                }
            }

            #[derive(Clone, FromLua, Debug)]
            pub struct AutocmdCallbackData {
                pub id: i64,
                pub event: String,
                pub group: Option<i64>,
                pub matc: Option<String>,
                pub buf: i64,
                pub file: Option<String>,
            }
        }

        nvim_exec(src: &str, output: bool) -> String

        /// Get all autocommands that match the corresponding `opts`.
        nvim_get_autocmds(opts: NvimGetAutocmdsOpts) -> Vec<AutoCmd> {
            #[derive(Builder, Default, ToLua)]
            #[builder(setter(strip_option))]
            #[builder(default)]
            pub struct NvimGetAutocmdsOpts {
                group: Option<StrI64>,
                event: Option<Vec<String>>,
                pattern: Option<Vec<String>>,
            }

            #[derive(Deserialize, Debug, Default, FromLua)]
            pub struct AutoCmd {
                pub id: Option<i64>,
                pub group: Option<i64>,
                pub group_name: Option<String>,
                pub desc: Option<String>,
                pub event: String,
                pub command: Option<String>,
                pub once: bool,
                pub pattern: String,
                pub buflocal: bool,
                pub buffer: Option<i64>,
            }
        }

        /// Find files in runtime direcories
        nvim_get_runtime_file(glob: &str, all: bool) -> Vec<String>

        nvim_set_keymap(mode: Mode, lhs: &str, rhs: &str, opts: NvimSetKeymapOpts) {
            #[derive(Clone, Copy, Debug, ToLua, Deserialize)]
            pub enum Mode {
                #[mlua(value = "")]
                #[serde(alias = "")]
                NormalVisualSelectOperatorPending,
                #[mlua(value = "n")]
                #[serde(alias = "n")]
                Normal,
                #[mlua(value = "v")]
                #[serde(alias = "v")]
                VisualSelect,
                #[mlua(value = "s")]
                #[serde(alias = "s")]
                Select,
                #[mlua(value = "x")]
                #[serde(alias = "x")]
                Visual,
                #[mlua(value = "o")]
                #[serde(alias = "o")]
                OperatorPending,
                #[mlua(value = "!")]
                #[serde(alias = "!")]
                InsertCommandline,
                #[mlua(value = "i")]
                #[serde(alias = "i")]
                Insert,
                #[mlua(value = "l")]
                #[serde(alias = "l")]
                InsertCommandLineLangArg,
                #[mlua(value = "c")]
                #[serde(alias = "c")]
                CommandLine,
                #[mlua(value = "t")]
                #[serde(alias = "t")]
                Terminal
            }
            input!{
                NvimSetKeymapOpts<'lua> {
                    pub nowait: bool,
                    pub silent: bool,
                    pub script: bool,
                    pub expr: bool,
                    pub unique: bool,
                    pub noremap: bool,
                    pub desc: Option<String>,
                    pub callback: Option<LuaFunction<'lua>>
                }
            }
        }
    }
}

#[derive(ToLua, From, Clone)]
pub enum StrI64 {
    Integer(i64),
    String(String),
}
impl<'lua> Command<'lua> {
    pub fn from_fn(
        lua: &'lua Lua,
        callback: impl Fn(&'lua Lua, CommandCallbackData) + 'static + Send,
    ) -> Self {
        Self::Lua(
            lua.create_function(move |lua, a| {
                callback(lua, a);
                Ok(())
            })
            .unwrap(),
        )
    }
    pub fn from_fn_async<F: Future<Output = ()> + 'lua>(
        lua: &'lua Lua,
        callback: impl (Fn(&'lua Lua, CommandCallbackData) -> F) + 'static + Send + Clone,
    ) -> Self {
        let f = lua
            .create_function(move |lua, a: CommandCallbackData| {
                let callback = callback.clone();
                let f = lua
                    .create_async_function(move |lua, a| callback(lua, a).map(Ok))
                    .unwrap()
                    .bind(a)
                    .unwrap();
                lua.load(chunk! {
                    local coroutine = coroutine.wrap($f)
                    local step = function() end
                    step = function()
                        if coroutine() ~= nil then
                            vim.schedule(step)
                        end
                    end
                    step()
                })
                .exec()
                .unwrap();
                Ok(())
            })
            .unwrap();
        Self::Lua(f)
    }
    pub fn from_fn_mut(
        lua: &'lua Lua,
        mut callback: impl FnMut(CommandCallbackData) + 'static + Send,
    ) -> Self {
        Self::Lua(
            lua.create_function_mut(move |_lua, a| {
                callback(a);
                Ok(())
            })
            .unwrap(),
        )
    }
}
