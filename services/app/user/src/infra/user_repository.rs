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
    user_id: String,
    name: String,
    email: String,
}

impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        User {
            model: Model {
                id: ID::from(row.id),
            },
            user_id: row.user_id,
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
    async fn create(&self, user: &User) -> Result<(), DomainError> {
        let mut conn = self.pool.acquire().await?;
        let result = InternalUserRepository::create(user, &mut conn).await?;
        Ok(result)
    }

    async fn find_by_id(&self, id: &ID) -> Result<Option<User>, DomainError> {
        let mut conn = self.pool.acquire().await?;
        let user = InternalUserRepository::find_by_id(id, &mut conn).await?;
        Ok(user)
    }

    async fn find_by_user_id(&self, user_id: &str) -> Result<Option<User>, DomainError> {
        let mut conn = self.pool.acquire().await?;
        let user = InternalUserRepository::find_by_user_id(user_id, &mut conn).await?;
        Ok(user)
    }
}

/// [Humble Object](https://martinfowler.com/bliki/HumbleObject.html)
pub(super) struct InternalUserRepository;

impl InternalUserRepository {
    pub(super) async fn create(user: &User, conn: &mut PgConnection) -> Result<(), DomainError> {
        sqlx::query("INSERT INTO users (id, user_id, name, email) VALUES ($1, $2, $3, $4)")
            .bind(user.model.id.as_str())
            .bind(&user.user_id)
            .bind(&user.name)
            .bind(&user.email)
            .execute(conn)
            .await?;
        Ok(())
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

    async fn find_by_user_id(
        user_id: &str,
        conn: &mut PgConnection,
    ) -> Result<Option<User>, DomainError> {
        let row: Option<UserRow> = sqlx::query_as("SELECT * FROM users WHERE user_id = $1")
            .bind(user_id)
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
