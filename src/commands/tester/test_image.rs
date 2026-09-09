use crate::cmd;
use crate::send_image;

const SAMPLE_IMAGE_URL: &str = "https://picsum.photos/seed/sora-rs/800/600";

cmd!(
    TestImage,
    name: "testimage",
    aliases: ["testimg"],
    category: "tester",
    execute: |ctx| {
        let source = if ctx.args.is_empty() {
            SAMPLE_IMAGE_URL.to_string()
        } else {
            ctx.args.join(" ")
        };

        send_image!(
            context: ctx,
            image_data: source,
            dst: ctx.info.source.chat,
            caption: "send_image! macro test ✅",
            reply: true
        )
        .await?;
    }
);
