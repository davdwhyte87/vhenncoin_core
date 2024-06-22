use core::fmt;



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


#[derive(Debug, PartialEq, Clone)]
pub enum MyErrorTypes {
    TransferWalletNotFound
}


#[derive(Clone, Debug)]
pub struct MyError{
   pub error: MyErrorTypes
}
impl fmt::Display for MyErrorTypes{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TransferWalletNotFound=> write!(f,"Transfer wallet not found")
        }
    }
}
impl fmt::Display for MyError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}
impl std::error::Error for MyError{}