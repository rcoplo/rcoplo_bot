use std::io::{Read, Write};
use serde::Deserializer;
use tokio::io::AsyncReadExt;

pub fn read_config() {
    let config_path =  crate::resource_path!("config","pluginConfig.yml");
    match std::fs::read_to_string(config_path) {
        Ok(d) => {

        }
        Err(_) => {}
    }
}

pub fn save_config(yml:serde_yaml::Value) -> crate::BotResult<()> {
    let config_path =  crate::resource_path!("config","pluginConfig.yml");
    let mut file = std::fs::OpenOptions::new().write(true).open(config_path)?;
    let mut string = serde_yaml::to_string(&yml)?;
    let mut v =  Vec::new();
    serde_yaml::to_writer(&mut v,&yml).expect("TODO: panic message");
    file.write_all(v.as_slice())?;
    Ok(())
}
#[test]
fn test(){
    save_config(x).unwrap();
}
