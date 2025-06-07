use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // user id
    pub exp: usize,         // expiration (UNIX timestamp)
    pub is_superuser: bool, // custom claim
}

#[derive(Clone)]
pub struct AuthUser {
    pub id: String,         // claim from token
    pub is_superuser: bool, // custom claim
}
