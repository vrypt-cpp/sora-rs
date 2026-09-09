use crate::cmd;
use crate::send_sticker;

const SAMPLE_STICKER_URL: &str = "https://www.gstatic.com/webp/gallery/1.webp";

cmd!(
    TestSticker,
    name: "teststicker",
    aliases: ["teststkr"],
    category: "tester",
    execute: |ctx| {
        let source = if ctx.args.is_empty() {
            SAMPLE_STICKER_URL.to_string()
        } else {
            ctx.args.join(" ")
        };

        send_sticker!(
            context: ctx,
            sticker_data: source,
            dst: ctx.info.source.chat,
            reply: true
        )
        .await?;
    }
);
