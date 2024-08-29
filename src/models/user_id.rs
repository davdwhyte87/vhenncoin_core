use serde::{Deserialize, Serialize};


#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserID {
    pub id: String,
    pub user_name:String, 
    pub password_hash:String, 
    pub date_created:String,
    pub recovery_answer:String,
    pub recovery_question:String,
    pub is_image_verification_linked:bool
}
