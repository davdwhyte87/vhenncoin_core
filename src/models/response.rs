use serde_derive::Serialize;

#[derive(Serialize)]
pub struct GenericResponse {
    pub message: String,
    pub code:i32
    
}

// 0 = error 1= success 4 = not found 6= unauthorized


