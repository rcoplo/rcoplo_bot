use base64::engine::Config;


use crate::plugin_config;
use crate::plugins::setu::SetuPluginConfig;

mod setu;
mod mc_status;
mod tool;

pub use mc_status::McServer;
pub fn all_message_event_plugins_matcher() -> Vec<nonebot_rs::matcher::Matcher<nonebot_rs::event::MessageEvent>> {
    vec![
        setu::SetuPlugin::run(),
        mc_status::McStatusPlugin::run(),
    ]
}
pub fn all_notice_event_plugins_matcher() -> Vec<nonebot_rs::matcher::Matcher<nonebot_rs::event::NoticeEvent>> {
    vec![
        tool::GroupPlugin::run(),
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

