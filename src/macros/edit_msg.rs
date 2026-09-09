#[macro_export]
macro_rules! edit_msg {
    (context: $ctx:expr, dst: $dst:expr, message_id: $msg_id:expr, text: $text:expr) => {{
        let new_content = whatsapp_rust::waproto::whatsapp::Message {
            conversation: Some($text.to_string()),
            ..Default::default()
        };

        $ctx.client
            .edit_message($dst.clone(), $msg_id, new_content)
    }};

    ($client:expr, dst: $dst:expr, message_id: $msg_id:expr, text: $text:expr) => {{
        let new_content = whatsapp_rust::waproto::whatsapp::Message {
            conversation: Some($text.to_string()),
            ..Default::default()
        };

        $client.edit_message($dst.clone(), $msg_id, new_content)
    }};
}
