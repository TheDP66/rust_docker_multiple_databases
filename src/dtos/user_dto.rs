use crate::models::user_model::UserModel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct UserDto {
    pub id: i32,
    pub name: String,
}

impl Into<UserDto> for UserModel {
    fn into(self) -> UserDto {
        UserDto {
            id: self.id,
            name: self.name,
        }
    }
}
