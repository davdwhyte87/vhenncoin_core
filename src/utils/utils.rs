

pub fn request_formatter(
    action:String, data:String, 
    message_singnature:String, 
    sender_node_public_key:String,
    is_braodcasted:String
)->String{
    return format!(
        "{}{}{}{}{}{}{}{}{}{}",
        action,
        r"\n",
        data,
        r"\n",
        message_singnature,
        r"\n",
        sender_node_public_key,
        r"\n",
        is_braodcasted,
        r"\n"
        );
}