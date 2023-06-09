use chrono::Timelike;
use nonebot_rs::matcher_vec;
use nonebot_rs::prelude::*;
use crate::plugins::mc_status::{get_minecraft_status_bedrock, get_minecraft_status_java, McServerImpl, McServerType};

#[event(bot_command = "/mc {mc_type} {name} {new_data}")]
async fn mc_status_mc(event: MessageEvent, m:Matcher<MessageEvent>, mc_type:Option<String>, name:Option<String>, new_data:Option<String>){
    match event {
        MessageEvent::Private(_) => {
            m.send_text("私聊不支持该指令喵...").await;
        }
        MessageEvent::Group(g) => {
           if m.is_admin().await{
               let group_id = g.group_id;
               let mc_server_impl = McServerImpl::default();
               match mc_type {
                   None => {
                       m.send_(MessageChain::new()
                           .text("可用子指令:\n")
                           .text(">    add\n")
                           .text(">    upname\n")
                           .text(">    upurl\n")
                           .text(">    uptype\n")
                           .text(">    d\n")
                           .text("使用: /mc add [name] url \n")
                           .text("此命令现仅超级管理员可用")
                           .build()).await;
                   }
                   Some(command) => {
                       if let (Some(name),Some(data)) = (name,new_data){
                           match command.as_str() {
                               "add" => {
                                   match mc_server_impl.new(name.to_uppercase().as_str(), data.as_str(),group_id, Ok(McServerType::JAVA)).await {
                                       Ok(_) => {
                                           m.send_text("添加成功喵!").await;
                                       }
                                       Err(err) => {
                                           m.send_text(err.to_string().as_str()).await;

                                       }
                                   }
                               }
                               "upname" => {
                                   match mc_server_impl.update_name_by_name_group_id(name.to_uppercase().as_str(), group_id, data.to_uppercase().as_str()).await {
                                       Ok(_) => {
                                           m.send_text("修改成功喵!").await;
                                       }
                                       Err(err) => {
                                           m.send_text(err.to_string().as_str()).await;
                                       }
                                   }
                               }
                               "upurl" => {
                                   match mc_server_impl.update_url_by_name_group_id(name.to_uppercase().as_str(), group_id, data.as_str()).await {
                                       Ok(_) => {
                                           m.send_text("修改成功喵!").await;

                                       }
                                       Err(err) => {
                                           m.send_text(err.to_string().as_str()).await;

                                       }
                                   }
                               }
                               "uptype" => {
                                   match mc_server_impl.update_server_type_by_name_group_id(name.to_uppercase().as_str(), group_id, McServerType::new(data.to_uppercase().as_str())).await {
                                       Ok(_) => {
                                           m.send_text("修改成功喵!").await;

                                       }
                                       Err(err) => {
                                           m.send_text(err.to_string().as_str()).await;

                                       }
                                   }
                               }
                               "d" => {
                                   match mc_server_impl.delete_server_by_name_group_id(name.to_uppercase().as_str(), group_id).await {
                                       Ok(_) => {
                                           m.send_text("删除成功喵!").await;

                                       }
                                       Err(err) => {
                                           m.send_text(err.to_string().as_str()).await;

                                       }
                                   }
                               }
                               _ => {
                                   m.send_text("没有这个子参数喵...").await;

                               }
                           }
                       }else {
                           m.send_text("参数不足喵...").await;

                       }
                   }
               }
            }else {
               m.at_text("你没有权限使用该指令喵...").await;
           }
        }
    }
    
}
#[event(bot_command = "/list {name}")]
async fn mc_status_list(event: MessageEvent,m:Matcher<MessageEvent>,name:Option<String>){
    match event {
        MessageEvent::Private(_) => {
            m.send_text("私聊不支持该指令喵...").await;
        }
        MessageEvent::Group(g) => {
            let mc_server_impl = McServerImpl::default();
            match name {
                None => {
                    match mc_server_impl.select_server_all_by_group_id(g.group_id).await {
                        None => {}
                        Some(v) => {
                            if v.is_empty() {
                                m.send_text("本群一个服务器都没绑定喵...").await;
                            }else {
                                let mut chain = MessageChain::new();
                                chain.text("当前可用服务器列表:\n");
                                for (i, server) in v.iter().enumerate() {
                                    chain.text(&format!("{}. {}\n", i + 1, server.name));
                                }
                                chain.text("可用指令:  /list {name}");
                                m.send_(chain.build()).await;
                            }
                        }
                    }
                }
                Some(name) => {
                    match mc_server_impl.select_server_by_name_group_id(name.to_uppercase().as_str(), g.group_id).await {
                        None => {
                            m.send_text("本群没有绑定这个简称喵...").await;
                        }
                        Some(mc_server) => {
                            match McServerType::new(mc_server.server_type.as_str()) {
                                Ok(server_type) => {
                                    match server_type {
                                        McServerType::JAVA => {
                                            match get_minecraft_status_java(mc_server.url.as_str()).await {
                                                Ok(status) => {
                                                    if status.online {
                                                        let mut chain = MessageChain::new();
                                                        let player = status.players.unwrap();
                                                        chain.text(&format!("{} Online: {}/{}\n", mc_server.name, player.online, player.max));
                                                        let vec = player.list
                                                            .iter()
                                                            .map(|list| {
                                                                list.name_raw.to_owned()
                                                            }).collect::<Vec<_>>();
                                                        if vec.len() == 0 {
                                                            chain.text("没有玩家在服务器喵...");
                                                        } else {
                                                            chain.text(&format!("{:?}", vec)
                                                                .replace("\"", "")
                                                                .replace("[", "")
                                                                .replace("]", ""));
                                                        }
                                                        let cache_time = chrono::NaiveDateTime::from_timestamp_millis(status.expires_at).unwrap_or_default();
                                                        let time = chrono::Utc::now().naive_utc();
                                                        if cache_time.minute() > time.minute() {
                                                            let duration = cache_time.time() - time.time();
                                                            if duration.num_seconds() < 55 {
                                                                chain.text(&format!("\n数据还剩{}秒刷新喵!", duration.num_seconds()));
                                                            }
                                                        }
                                                        m.send_(chain.build()).await;
                                                    } else {
                                                        m.send_text("服务器当前不在线喵...").await;
                                                    }
                                                }
                                                Err(err) => {
                                                    m.send_text(err.to_string().as_str()).await;
                                                }
                                            }
                                        }
                                        McServerType::Bedrock => {
                                            match get_minecraft_status_bedrock(mc_server.url.as_str()).await {
                                                Ok(status) => {
                                                    if status.online {
                                                        let mut chain = MessageChain::new();
                                                        let player = status.players.unwrap();
                                                        chain.text(&format!("{} Players: {}/{}\n", mc_server.name, player.online, player.max));
                                                        chain.text("Bedrock版无法获取到玩家列表喵!");
                                                        let cache_time = chrono::NaiveDateTime::from_timestamp_millis(status.expires_at).unwrap_or_default();
                                                        let time = chrono::Utc::now().naive_utc();
                                                        if cache_time.minute() > time.minute() {
                                                            let duration = cache_time.time() - time.time();
                                                            if duration.num_seconds() < 55 {
                                                                chain.text(&format!("\n数据还剩{}秒刷新喵!", duration.num_seconds()));
                                                            }
                                                        }
                                                        m.send_(chain.build()).await;
                                                    } else {
                                                        m.send_text("服务器当前不在线喵...").await;
                                                    }
                                                }
                                                Err(err) => {
                                                    m.send_text(err.to_string().as_str()).await;
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(err) => {
                                    m.send_text(err.to_string().as_str()).await;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

matcher_vec!(MessageEvent,{
    vec![
        Matcher::new("mc_status",mc_status_mc::default()),
        Matcher::new("mc_status_list",mc_status_list::default()),
    ]
});