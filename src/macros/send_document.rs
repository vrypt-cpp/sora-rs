#[macro_export]
macro_rules! send_document {
    (
        context: $ctx:expr,
        document_data: $data:expr,
        dst: $dst:expr,
        file_name: $file_name:expr,
        reply: $is_reply:expr
        $(, mimetype: $mimetype:expr)?
        $(, caption: $caption:expr)?
        $(, config_context: $config_fn:expr)?
    ) => {{
        async {
            use whatsapp_rust::wacore::proto_helpers::build_quote_context_with_info;
            use whatsapp_rust::waproto::whatsapp::{Message, message::DocumentMessage, ContextInfo};
            use whatsapp_rust::wacore::download::MediaType;
            use whatsapp_rust::UploadOptions;

            let client = &$ctx.client;
            let info = &$ctx.info;
            let state = &$ctx.state;

            let raw_data: Vec<u8> = $data.into();
            let document_bytes = $crate::utils::get_media_bytes(std::sync::Arc::clone(state), raw_data).await?;
            let upload = client.upload(document_bytes, MediaType::Document, UploadOptions::default()).await?;

            let mut context_info = if $is_reply {
                let mut ctx_info = build_quote_context_with_info(
                    &info.id,
                    &info.source.sender,
                    &info.source.chat,
                    &info.source.chat,
                    $ctx.msg,
                );
                ctx_info.mentioned_jid = vec![info.source.sender.to_non_ad().to_string()];
                ctx_info
            } else {
                ContextInfo::default()
            };

            let expiration = state.get_expiration(&$dst.to_string());
            if expiration > 0 {
                context_info.expiration = Some(expiration);
            }

            $(
                ($config_fn)(&mut context_info);
            )?

            context_info.remote_jid = Some($ctx.info.source.chat.to_string());

            #[allow(unused_mut, unused_assignments)]
            let mut mimetype = "application/octet-stream".to_string();
            $(
                mimetype = $mimetype.to_string();
            )?

            #[allow(unused_mut, unused_assignments)]
            let mut caption: Option<String> = None;
            $(
                caption = Some($caption.to_string());
            )?

            let document_msg = Message {
                document_message: whatsapp_rust::buffa::MessageField::some(DocumentMessage {
                    url: Some(upload.url),
                    direct_path: Some(upload.direct_path),
                    media_key: Some(upload.media_key.to_vec()),
                    file_sha256: Some(upload.file_sha256.to_vec()),
                    file_enc_sha256: Some(upload.file_enc_sha256.to_vec()),
                    file_length: Some(upload.file_length),
                    media_key_timestamp: Some(upload.media_key_timestamp),
                    file_name: Some($file_name.to_string()),
                    mimetype: Some(mimetype),
                    caption,
                    context_info: whatsapp_rust::buffa::MessageField::some(context_info),
                    ..Default::default()
                }),
                ..Default::default()
            };

            client.send_message($dst.clone(), document_msg).await
        }
    }};
}
