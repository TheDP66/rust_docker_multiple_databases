use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct UserModel {
    pub id: i32,
    pub name: String,
}
