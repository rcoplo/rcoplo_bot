use rbatis::{crud, impl_select};
use crate::{BotError, BotResult, pool};
use crate::util::http_get;

pub mod mc_status;
// pub use mc_status::McStatusPlugin;

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct McServer {
    pub id: Option<i32>,
    pub group_id: i64,
    pub name: String,
    pub url: String,
    pub server_type: String,
}
#[derive(Default)]
pub struct McServerImpl;

crud!(McServer {});
impl_select!(McServer{select_server_by_name(name:&str,group_id:i64) -> Option => "`where name = #{name} and group_id = #{group_id}`"});

pub enum McServerType {
    JAVA,
    Bedrock,
}

impl McServerType {
    pub fn new(server_type: &str) -> BotResult<McServerType> {
        match server_type {
            "JE" => Ok(McServerType::JAVA),
            "BE" => Ok(McServerType::Bedrock),
            _ => Err(BotError::from(format!("\"{}\" 这个类型不存在喵... 只有:[JE,BE]", server_type)))
        }
    }
}

impl ToString for McServerType {
    fn to_string(&self) -> String {
        match self {
            McServerType::JAVA => "JE".to_string(),
            McServerType::Bedrock => "BE".to_string()
        }
    }
}

impl McServerImpl {
    pub async fn new(&self, name: &str, url: &str, group_id: i64, server_type: BotResult<McServerType>) -> BotResult<()> {
        match self.select_server_by_name_group_id(name, group_id).await {
            None => {
                McServer::insert(pool!(), &McServer {
                    id: None,
                    group_id,
                    name: name.to_string(),
                    url: url.to_string(),
                    server_type: {
                        match server_type {
                            Ok(data) => data.to_string(),
                            Err(err) => return Err(err),
                        }
                    },
                }).await?;
                Ok(())
            }
            Some(_) => {
                Err(BotError::from("本群已有该服务器的简称喵..."))
            }
        }
    }

    pub async fn select_server_by_name_group_id(&self, name: &str, group_id: i64) -> Option<McServer> {
        let mc_server = McServer::select_server_by_name(pool!(), name, group_id).await.ok()?;
        mc_server
    }

    pub async fn select_server_all_by_group_id(&self, group_id: i64) -> Option<Vec<McServer>> {
        let mc_server = McServer::select_by_column(pool!(), "group_id", group_id).await.ok();
        mc_server
    }

    pub async fn update_name_by_name_group_id(&self, name: &str, group_id: i64, new_name: &str) -> BotResult<()> {
        match self.select_server_by_name_group_id(name, group_id).await {
            None => {
                Err(BotError::from("本群并没有绑定这个服务器简称喵..."))
            }
            Some(mc_server) => {
                McServer::update_by_column(pool!(), &McServer {
                    name: new_name.to_string(),
                    ..mc_server
                }, "id").await?;
                Ok(())
            }
        }
    }

    pub async fn update_url_by_name_group_id(&self, name: &str, group_id: i64, new_url: &str) -> BotResult<()> {
        match self.select_server_by_name_group_id(name, group_id).await {
            None => {
                Err(BotError::from("本群并没有绑定这个服务器简称喵..."))
            }
            Some(mc_server) => {
                McServer::update_by_column(pool!(), &McServer {
                    url: new_url.to_string(),
                    ..mc_server
                }, "id").await?;
                Ok(())
            }
        }
    }
    pub async fn update_server_type_by_name_group_id(&self, name: &str, group_id: i64, new_server_type: BotResult<McServerType>) -> BotResult<()> {
        match self.select_server_by_name_group_id(name, group_id).await {
            None => {
                Err(BotError::from("本群并没有绑定这个服务器简称喵..."))
            }
            Some(mc_server) => {
                McServer::update_by_column(pool!(), &McServer {
                    server_type: {
                        match new_server_type {
                            Ok(data) => data.to_string(),
                            Err(err) => return Err(err),
                        }
                    },
                    ..mc_server
                }, "id").await?;
                Ok(())
            }
        }
    }

    pub async fn delete_server_by_name_group_id(&self, name: &str, group_id: i64) -> BotResult<()> {
        match self.select_server_by_name_group_id(name, group_id).await {
            None => {
                Err(BotError::from("本群并没有绑定这个服务器简称喵..."))
            }
            Some(mc_server) => {
                McServer::delete_by_column(pool!(), "name", mc_server.name).await?;
                Ok(())
            }
        }
    }
}


const STATUS_API_JAVA: &str = "https://api.mcstatus.io/v2/status/java/";
const STATUS_API_BEDROCK: &str = "https://api.mcstatus.io/v2/status/bedrock/";


pub async fn get_minecraft_status_java(url: &str) -> BotResult<McStatusJava> {
    let data = http_get(format!("{}{}", STATUS_API_JAVA, url).as_str()).await?;
    match serde_json::from_str::<McStatusJava>(data.as_str()) {
        Ok(data) => Ok(data),
        Err(_) => {
            let data = http_get(&format!("https://api.mcsrvstat.us/simple/{}", url)).await?;
            match data.as_str() {
                "" => {}
                _ => {}
            }
            Err(BotError::from(format!("获取服务器信息失败喵...,")))
        },

    }
}

pub async fn get_minecraft_status_bedrock(url: &str) -> BotResult<McStatusBedrock> {
    let data = http_get(format!("{}{}", STATUS_API_BEDROCK, url).as_str()).await?;
    match serde_json::from_str::<McStatusBedrock>(data.as_str()) {
        Ok(data) => Ok(data),
        Err(_) => Err(BotError::from(format!("获取服务器信息失败喵... "))),
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct McStatusJava {
    pub online: bool,
    pub host: String,
    pub port: i32,
    pub eula_blocked: bool,
    pub retrieved_at: i64,
    pub expires_at: i64,
    pub version: Option<McStatusVersionJava>,
    pub players: Option<McStatusPlayersJava>,
    pub motd: Option<McStatusMotdJava>,
    pub icon: Option<Option<String>>,
    pub mods: Option<Vec<McStatusModsJava>>,

}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct McStatusVersionJava {
    pub name_raw: String,
    pub name_clean: String,
    pub name_html: String,
    pub protocol: i32,

}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct McStatusPlayersJava {
    pub online: i32,
    pub max: i32,
    pub list: Vec<McStatusListJava>,

}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct McStatusListJava {
    pub uuid: String,
    pub name_raw: String,
    pub name_clean: String,
    pub name_html: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct McStatusMotdJava {
    pub raw: String,
    pub clean: String,
    pub html: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct McStatusModsJava {
    pub name: String,
    pub version: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct McStatusBedrock {
    pub online: bool,
    pub host: String,
    pub port: i32,
    pub eula_blocked: bool,
    pub retrieved_at: i64,
    pub expires_at: i64,
    pub version: Option<McStatusVersionBedrock>,
    pub players: Option<McStatusPlayersBedrock>,
    pub motd: Option<McStatusMotdBedrock>,
    pub gamemode: Option<String>,
    pub server_id: Option<String>,
    pub edition: Option<String>,

}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct McStatusVersionBedrock {
    pub name: String,
    pub version: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct McStatusPlayersBedrock {
    pub online: String,
    pub max: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct McStatusMotdBedrock {
    pub raw: String,
    pub clean: String,
    pub html: String,
}

