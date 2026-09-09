use crate::cmd;
use crate::send_document;

const SAMPLE_DOCUMENT_URL: &str = "https://www.w3.org/WAI/ER/tests/xhtml/testfiles/resources/pdf/dummy.pdf";

cmd!(
    TestDocument,
    name: "testdocument",
    aliases: ["testdoc"],
    category: "tester",
    execute: |ctx| {
        let source = if ctx.args.is_empty() {
            SAMPLE_DOCUMENT_URL.to_string()
        } else {
            ctx.args.join(" ")
        };

        send_document!(
            context: ctx,
            document_data: source,
            dst: ctx.info.source.chat,
            file_name: "send_document_test.pdf",
            reply: true,
            mimetype: "application/pdf",
            caption: "send_document! macro test ✅"
        )
        .await?;
    }
);
