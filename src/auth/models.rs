use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    pub code: String
}

#[derive(Deserialize, Clone)]
pub struct UserProfile {
    pub email: String
}

#[derive(Deserialize, Clone)]
pub struct UserExtension {
    pub id: i32,
    pub email: String
}
