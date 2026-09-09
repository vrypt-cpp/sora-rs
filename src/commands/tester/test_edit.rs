use crate::cmd;
use crate::edit_msg;

cmd!(
    TestEdit,
    name: "testedit",
    aliases: [],
    category: "tester",
    execute: |ctx| {
        let msg_id = ctx.reply("edit_msg! macro test — before edit").await?;

        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        edit_msg!(
            context: ctx,
            dst: ctx.info.source.chat,
            message_id: msg_id,
            text: "edit_msg! macro test ✅ — after edit"
        )
        .await?;
    }
);
