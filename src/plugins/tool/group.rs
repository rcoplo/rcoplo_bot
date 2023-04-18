use nonebot_rs::{async_trait, on_event, on_match_all};
use nonebot_rs::event::{NoticeEvent, NoticeType};
use nonebot_rs::matcher::{Handler, Matcher};
use nonebot_rs::message::MessageChain;

pub struct  GroupPlugin ;

on_event!(NoticeEvent,GroupPlugin,{
    Matcher::new("GroupPlugin",GroupPlugin::new())
});

#[async_trait]
impl Handler<NoticeEvent> for GroupPlugin{
    on_match_all!(NoticeEvent);
    async fn handle(&self, e: NoticeEvent, m: Matcher<NoticeEvent>) {
        match e.notice_type {
            NoticeType::GroupDecrease => {
                let info = m.get_stranger_info(e.user_id, false).await.unwrap();
                m.send_group_msg(e.group_id.unwrap(),MessageChain::new()
                    .text(format!("{}离开了我们喵...",info.nickname)).build(),false).await;
                return;
            }
            NoticeType::GroupIncrease => {
                m.send_group_msg(e.group_id.unwrap(),MessageChain::new()
                    .at(e.user_id)
                    .text(format!("欢迎新绒布球入群喵!")).build(),false).await;
                return;
            }
            _ => {}
        }
    }
}