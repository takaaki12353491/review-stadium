use crate::domain::{user::User, user_repository::UserRepository};
use async_trait::async_trait;
use common::{
    error::DomainError,
    model::{Model, ID},
    string::StringExt,
};
use sqlx::{PgConnection, PgPool};

#[derive(sqlx::FromRow)]
struct UserRow {
    id: String,
    id_name: String,
    name: String,
    email: String,
}

impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        User {
            model: Model {
                id: ID::from(row.id),
            },
            id_name: row.id_name,
            name: row.name,
            email: row.email.to_option(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserRepositoryImpl {
    pool: PgPool,
}

impl UserRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create(&self, id_name: &str, name: &str, email: &str) -> Result<User, DomainError> {
        let mut conn = self.pool.acquire().await?;
        InternalUserRepository::create(id_name, name, email, &mut conn).await
    }

    async fn find_by_id(&self, id: &ID) -> Result<Option<User>, DomainError> {
        let mut conn = self.pool.acquire().await?;
        let user = InternalUserRepository::find_by_id(id, &mut conn).await?;
        Ok(user)
    }

    async fn find_by_id_name(&self, id_name: &str) -> Result<Option<User>, DomainError> {
        let mut conn = self.pool.acquire().await?;
        let user = InternalUserRepository::find_by_id_name(id_name, &mut conn).await?;
        Ok(user)
    }
}

/// [Humble Object](https://martinfowler.com/bliki/HumbleObject.html)
pub(super) struct InternalUserRepository;

impl InternalUserRepository {
    async fn create(
        id_name: &str,
        name: &str,
        email: &str,
        conn: &mut PgConnection,
    ) -> Result<User, DomainError> {
        let user_row: UserRow = sqlx::query_as(
            "INSERT INTO users (id_name, name, email) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(id_name)
        .bind(name)
        .bind(email)
        .fetch_one(conn)
        .await?;
        Ok(User::from(user_row))
    }

    async fn find_by_id(id: &ID, conn: &mut PgConnection) -> Result<Option<User>, DomainError> {
        let row: Option<UserRow> = sqlx::query_as("SELECT * FROM users WHERE id = $1")
            .bind(id.as_str())
            .fetch_optional(conn)
            .await?;

        match row {
            Some(user_row) => Ok(Some(user_row.into())),
            None => Ok(None),
        }
    }

    async fn find_by_id_name(
        id_name: &str,
        conn: &mut PgConnection,
    ) -> Result<Option<User>, DomainError> {
        let row: Option<UserRow> = sqlx::query_as("SELECT * FROM users WHERE id_name = $1")
            .bind(id_name)
            .fetch_optional(conn)
            .await?;

        match row {
            Some(user_row) => Ok(Some(user_row.into())),
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    // TODO: InternalUserRepositoryのテスト
}
