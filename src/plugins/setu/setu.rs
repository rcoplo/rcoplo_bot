use base64::Engine;
use nonebot_rs::matcher_vec;
use nonebot_rs::prelude::{ At, MessageChain, MessageChainTrait, MessageEvent, event, Matcher};

use crate::BotResult;
use crate::plugins::setu::LoliconApiBuilder;
use crate::util::http_get_image;

#[event(bot_command = "/色图 {param}")]
async fn setu(_e:MessageEvent,m:Matcher<MessageEvent>,param:Vec<String>) {
    let setu = setu_builder(&param, false);
    match setu_send(&m, setu).await {
        Ok(_) => {}
        Err(err) => {
            m.send_text(&err.to_string()).await;
        }
    };
}

#[event(bot_command = "/色图r {param}")]
async fn setur(_e:MessageEvent,m:Matcher<MessageEvent>,param:Vec<String>) {
    let setu = setu_builder(&param, true);
    match setu_send(&m, setu).await {
        Ok(_) => {}
        Err(err) => {
            m.send_text(&err.to_string()).await;
        }
    };
}

matcher_vec!(MessageEvent,{
    vec![
        Matcher::new("setu",setu::default()),
        Matcher::new("setur",setur::default()),
    ]
});

enum SetuType {
    Array(LoliconApiBuilder),
    Single(LoliconApiBuilder),
}
async fn setu_send(m: &Matcher<MessageEvent>, setu: SetuType) -> BotResult<()> {
    let mut chain:MessageChain;
    match setu {
        SetuType::Array(s) => {
            match LoliconApiBuilder::get_array(&s).await {
                Ok(setu) => {
                    for setu in setu {
                        chain = MessageChain::new()
                            .text(format!("title: {}\n", setu.title))
                            .text(format!("pid: {}\n", setu.pid))
                            .text(format!("author: {}\n", setu.author))
                            .image(&match http_get_image(&setu.urls.original).await {
                                Ok(d) => {format!("base64://{}",d)}
                                Err(err) => {
                                    format!("base64://{}", base64::engine::general_purpose::STANDARD.encode(err.to_string()))
                                }
                            })
                            .build();
                        match m.send(chain).await {
                            None => {
                                m.send_text(&format!("这张色图失败喵... Pid:{}",setu.pid)).await;
                            }
                            Some(message_id) => {
                                de_msg(message_id.message_id,&m);
                            }
                        };
                    }
                }
                Err(err) => {
                    m.send_text(&err.to_string()).await;
                }
            }
        }
        SetuType::Single(s) => {
            match LoliconApiBuilder::get(&s).await {
                Ok(setu) => {
                     chain = MessageChain::new()
                        .text(format!("title: {}\n", setu.title))
                        .text(format!("pid: {}\n", setu.pid))
                        .text(format!("author: {}\n", setu.author))
                         .image(&match http_get_image(&setu.urls.original).await {
                             Ok(d) => {format!("base64://{}",d)}
                             Err(err) => {
                                 format!("base64://{}",base64::engine::general_purpose::STANDARD.encode(err.to_string()))
                             }
                         })
                        .build();
                    match m.send(chain).await {
                        None => {
                            m.send_text(&format!("发送色图失败喵... Pid:{}",setu.pid)).await;
                        }
                        Some(message_id) => {
                            de_msg(message_id.message_id,&m);
                        }
                    };
                }
                Err(err) => {
                    m.send_text(&err.to_string()).await;
                }
            }
        }
    }
    Ok(())
}

fn de_msg(message_id:i32,m: &Matcher<MessageEvent>) {
    let m = m.clone();
    tokio::spawn(async move{
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        m.delete_msg(message_id).await;
    });
}
fn setu_builder(data: &Vec<String>, is_r18: bool) -> SetuType {
    let mut builder = LoliconApiBuilder::new();
    if is_r18 {
        if !data.is_empty() {
            match data.first().unwrap().parse::<i8>() {
                Ok(n) => {
                    if data.len() > 1 {
                        let mut data = data.clone();
                        data.remove(0);
                        SetuType::Array(builder.tag(data).num(n).r18().build())
                    } else {
                        if n <= 20 {
                            SetuType::Array(builder.num(n).r18().build())
                        } else {
                            SetuType::Single(builder.r18().build())
                        }
                    }
                }
                Err(_) => {
                    SetuType::Single(builder.tag(data.clone()).r18().build())
                }
            }
        } else {
            SetuType::Single(builder.r18().build())
        }
    } else {
        if !data.is_empty() {
            match data.first().unwrap().parse::<i8>() {
                Ok(n) => {
                    if data.len() > 1 {
                        let mut data = data.clone();
                        data.remove(0);
                        SetuType::Array(builder.tag(data).num(n).no_r18().build())
                    } else {
                        if n <= 20 {
                            SetuType::Array(builder.num(n).no_r18().build())
                        } else {
                            SetuType::Single(builder.no_r18().build())
                        }
                    }
                }
                Err(_) => {
                    SetuType::Single(builder.tag(data.clone()).no_r18().build())
                }
            }
        } else {
            SetuType::Single(builder.no_r18().build())
        }
    }
}
