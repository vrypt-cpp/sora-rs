#[macro_export]
macro_rules! revoke_msg {
    (context: $ctx:expr, dst: $dst:expr, message_id: $msg_id:expr) => {
        $ctx.client.revoke_message(
            $dst.clone(),
            $msg_id,
            whatsapp_rust::send::RevokeType::Sender,
        )
    };

    (context: $ctx:expr, dst: $dst:expr, message_id: $msg_id:expr, original_sender: $sender:expr) => {
        $ctx.client.revoke_message(
            $dst.clone(),
            $msg_id,
            whatsapp_rust::send::RevokeType::Admin {
                original_sender: $sender,
            },
        )
    };

    ($client:expr, dst: $dst:expr, message_id: $msg_id:expr) => {
        $client.revoke_message(
            $dst.clone(),
            $msg_id,
            whatsapp_rust::send::RevokeType::Sender,
        )
    };
}
