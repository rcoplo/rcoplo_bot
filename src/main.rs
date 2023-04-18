use rcoplo_bot::BOT_CONTEXT;

#[tokio::main]
async fn main() {
    BOT_CONTEXT.init_pool().await;
    let mut nb = nonebot_rs::Nonebot::new();

    let mut matchers = nonebot_rs::Matchers::new_empty();
    matchers.add_message_matchers(rcoplo_bot::plugins::all_message_event_plugins_matcher());
    matchers.add_notice_matchers(rcoplo_bot::plugins::all_notice_event_plugins_matcher());
    nb.add_plugin(matchers);

    let mut scheduler = nonebot_rs::scheduler::Scheduler::new().await;
    scheduler.add_task(Box::new(rcoplo_bot::TempFileClean)).await;
    nb.add_plugin(scheduler);

    nb.async_run().await;
}


