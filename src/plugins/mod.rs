use nonebot_rs::prelude::{Matcher, MessageEvent, NoticeEvent};
use crate::plugin_config;
use crate::plugins::setu::SetuPluginConfig;

mod setu;
mod mc_status;
mod tool;

pub use mc_status::McServer;

pub fn all_message_event_plugins_matcher() -> Vec<Matcher<MessageEvent>> {
    let mut vec = vec![];
    vec.extend(mc_status::mc_status::matcher());
    vec.extend(setu::setu::matcher());
    vec
}

pub fn all_notice_event_plugins_matcher() -> Vec<Matcher<NoticeEvent>> {
    vec![
        tool::group::matcher(),
    ]
}

pub fn all_plugins_settings() -> Vec<impl crate::PluginSettings> {

    vec![
        setu::SetuPluginSettings
    ]
}

plugin_config!(
    setu: SetuPluginConfig
);

