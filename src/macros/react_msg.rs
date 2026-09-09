#[macro_export]
macro_rules! react_msg {
    (context: $ctx:expr, dst: $dst:expr, message_id: $msg_id:expr, emoji: $emoji:expr) => {
        $crate::react_msg!(
            $ctx.client,
            dst: $dst,
            message_id: $msg_id,
            from_me: false,
            participant: None,
            emoji: $emoji
        )
    };

    (context: $ctx:expr, dst: $dst:expr, message_id: $msg_id:expr, participant: $participant:expr, emoji: $emoji:expr) => {
        $crate::react_msg!(
            $ctx.client,
            dst: $dst,
            message_id: $msg_id,
            from_me: false,
            participant: Some($participant.to_string()),
            emoji: $emoji
        )
    };

    ($client:expr, dst: $dst:expr, message_id: $msg_id:expr, from_me: $from_me:expr, participant: $participant:expr, emoji: $emoji:expr) => {{
        let target_key = whatsapp_rust::waproto::whatsapp::MessageKey {
            remote_jid: Some($dst.to_string()),
            from_me: Some($from_me),
            id: Some($msg_id.to_string()),
            participant: $participant,
        };

        $client.send_reaction($dst.clone(), target_key, $emoji)
    }};
}
