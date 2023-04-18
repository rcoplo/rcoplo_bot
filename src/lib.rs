#![feature(file_create_new)]

pub mod plugins;
mod api;
mod base_plugins;
pub mod util;
mod error;
mod config;

pub use error::{
    BotResult,BotError
};

pub use base_plugins::{
    TempFileClean
};

pub use util::macros::{
    PluginSettings
};

use crate::plugins::McServer;

#[macro_export]
macro_rules! pool {
    () => {
        Box::leak(Box::new($crate::BOT_CONTEXT.rb.clone()))
    };

}

bot_context!(
    McServer => "mc_server"
);