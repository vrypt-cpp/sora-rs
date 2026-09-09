use crate::cmd;
use crate::react_msg;

cmd!(
    TestReact,
    name: "testreact",
    aliases: [],
    category: "tester",
    execute: |ctx| {
        let (target_id, participant) = if let Some(ext) = ctx.msg.extended_text_message.as_option()
            && let Some(ci) = ext.context_info.as_option()
            && let Some(quoted_id) = ci.stanza_id.as_ref()
        {
            (quoted_id.clone(), ci.participant.clone())
        } else {
            (ctx.info.id.to_string(), None)
        };

        if let Some(participant) = participant {
            react_msg!(
                context: ctx,
                dst: ctx.info.source.chat,
                message_id: target_id,
                participant: participant,
                emoji: "🧪"
            )
            .await?;
        } else {
            react_msg!(
                context: ctx,
                dst: ctx.info.source.chat,
                message_id: target_id,
                emoji: "🧪"
            )
            .await?;
        }

        ctx.reply("react_msg! macro test ✅").await?;
    }
);
