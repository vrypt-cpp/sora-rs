use crate::cmd;
use crate::send_audio;

const SAMPLE_AUDIO_URL: &str = "https://samplelib.com/lib/preview/mp3/sample-15s.mp3";

cmd!(
    TestAudio,
    name: "testaudio",
    aliases: [],
    category: "tester",
    execute: |ctx| {
        let source = if ctx.args.is_empty() {
            SAMPLE_AUDIO_URL.to_string()
        } else {
            ctx.args.join(" ")
        };

        send_audio!(
            context: ctx,
            audio_data: source,
            dst: ctx.info.source.chat,
            reply: true
        )
        .await?;

        ctx.reply("send_audio! macro test ✅ (media_key_timestamp + streaming_sidecar populated)")
            .await?;
    }
);
