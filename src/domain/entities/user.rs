use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Hash, Eq, PartialEq, Debug)]
pub struct User {
    pub uuid: Uuid,
    pub name: String,
    pub password_hash: String,
    pub email: String,
    pub file_path: Option<String>,
}
