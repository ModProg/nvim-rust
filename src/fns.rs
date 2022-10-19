use crate::api;
use mlua::prelude::*;

api! {
    /// `vim.api.*`
    Fns("https://neovim.io/doc/user/builtin.html" # "builtin-functions") {
        getreg(regname: &str) -> String
        stdpath(path_type: PathType) -> String {
            #[derive(Debug, Clone, Copy, ToLua)]
            pub enum PathType {
                #[mlua(value="cache")]
                Cache,
                #[mlua(value="config")]
                Config,
                #[mlua(value="data")]
                Data,
                #[mlua(value="log")]
                Log,
                #[mlua(value="state")]
                State,
                // TODO config_dirs, data_dirs
            }
        }
    }
}
