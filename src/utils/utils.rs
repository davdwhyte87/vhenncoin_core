use core::{fmt, num};



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



// this helps us round up numbers to 3 decimal places
pub fn round_number(number:f32)->f32{
    let y = (number * 1000.0).round() / 1000.0;
    return y ;
}


// helps make sure we have ascii characters 
pub fn is_ascii(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii())
}

pub fn validate_user_name(user_name:&str)->bool{
            // check if there is white space in the wallet address
            if user_name.contains(char::is_whitespace){

                return false;  
            }
            // make sure address is lower case
            if user_name.chars().all(|c|  !c.is_alphabetic() || c.is_lowercase()) {
                return true
            }else{
                return false
            }

}