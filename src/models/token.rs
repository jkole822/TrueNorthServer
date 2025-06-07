use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,          // user id
    pub exp: usize,         // expiration (UNIX timestamp)
    pub is_superuser: bool, // custom claim
}

#[derive(Clone)]
pub struct AuthUser {
    pub id: Uuid,           // claim from token
    pub is_superuser: bool, // custom claim
}
