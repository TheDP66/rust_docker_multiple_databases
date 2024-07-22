use sqlx::{Pool, Postgres};

use crate::{dtos::user_dto::UserDto, models::user_model::UserModel};

#[derive(Debug)]
pub struct UserService {
    pool: Pool<Postgres>,
}

impl UserService {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn get_user_service(&self) -> Result<Vec<UserDto>, sqlx::Error> {
        let result = sqlx::query_as::<_, UserModel>(
            "SELECT *
        FROM \"user\"",
        )
        .fetch_all(&self.pool)
        .await?;

        let mut user_dtos: Vec<UserDto> = vec![];

        for user in result {
            let user_dto = UserDto {
                id: user.id,
                name: user.name,
            };

            user_dtos.push(user_dto);
        }

        Ok(user_dtos)
    }
}
