use crate::cmd;
use crate::send_video;

const SAMPLE_VIDEO_URL: &str = "https://samplelib.com/lib/preview/mp4/sample-5s.mp4";

cmd!(
    TestVideo,
    name: "testvideo",
    aliases: [],
    category: "tester",
    execute: |ctx| {
        let source = if ctx.args.is_empty() {
            SAMPLE_VIDEO_URL.to_string()
        } else {
            ctx.args.join(" ")
        };

        send_video!(
            context: ctx,
            video_data: source,
            dst: ctx.info.source.chat,
            caption: "send_video! macro test ✅ (media_key_timestamp + streaming_sidecar populated)",
            reply: true
        )
        .await?;
    }
);
