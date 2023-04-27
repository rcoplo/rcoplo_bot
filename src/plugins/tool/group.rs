use nonebot_rs::matcher;
use nonebot_rs::prelude::*;

#[event]
async fn group_decrease_increase(e:NoticeEvent,m:Matcher<NoticeEvent>){
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

matcher!(NoticeEvent,{
    Matcher::new("group_decrease_increase",group_decrease_increase{})
});