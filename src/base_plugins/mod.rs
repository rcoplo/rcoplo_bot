mod config;


use base64::engine::Config;
use og_image_writer::style::{AlignItems, JustifyContent};
use crate::base_plugins_config;


base_plugins_config!();



#[cfg(test)]
mod test{

    pub struct TestPluginSettings;

    impl Default for TestPluginSettings{
        fn default() -> Self {
            todo!()
        }
    }
}

/// 临时文件夹每天晚上凌晨清理一次
pub struct TempFileClean;
impl nonebot_rs::scheduler::ScheduledJob for TempFileClean{
    fn name(&self) -> &'static str {
        "TempFileClean"
    }
    fn cron(&self) -> &'static str {
        "0 0 00 * * ?"
    }
    fn call(&self, _: std::sync::Arc<nonebot_rs::Bot>) -> std::pin::Pin<Box<dyn std::future::Future<Output=()> + Send + 'static>> {
        Box::pin(async move{
            let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            path.push("resources");
            path.push("tmp");
            match std::fs::remove_dir_all(&path) {
                Ok(_) => {
                    std::fs::create_dir_all(path).expect("创建临时目录失败!");
                    nonebot_rs::log::event!(nonebot_rs::log::Level::INFO,"临时文件删除成功!");
                }
                Err(err) => {
                    nonebot_rs::log::event!(nonebot_rs::log::Level::INFO,"临时文件删除失败! 错误:{}",err);
                }
            }
        })
    }
}

pub struct PluginHelpBuilder{
    inner:og_image_writer::writer::OGImageWriter,
}

impl PluginHelpBuilder {
    pub fn new() -> PluginHelpBuilder {
        Self{
            inner: og_image_writer::writer::OGImageWriter::new(og_image_writer::style::WindowStyle{
                background_color: Some(og_image_writer::style::Rgba([255,255,255,255])),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            }).unwrap(),
        }
    }
}