use crate::cmd;
use crate::revoke_msg;

cmd!(
    TestRevoke,
    name: "testrevoke",
    aliases: [],
    category: "tester",
    execute: |ctx| {
        let msg_id = ctx
            .reply("revoke_msg! macro test — this message will be revoked shortly")
            .await?;

        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        revoke_msg!(
            context: ctx,
            dst: ctx.info.source.chat,
            message_id: msg_id
        )
        .await?;
    }
);
