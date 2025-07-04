use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
}

impl User {
    pub async fn create(pool: &sqlx::PgPool, name: &str) -> anyhow::Result<Self> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (name)
            VALUES ($1)
            RETURNING id, name
            "#,
            name
        )
        .fetch_one(pool)
        .await?;
        Ok(user)
    }

    pub async fn get(pool: &sqlx::PgPool, id: i32) -> anyhow::Result<Option<Self>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, name FROM users WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;
        Ok(user)
    }

    pub async fn update(pool: &sqlx::PgPool, id: i32, name: &str) -> anyhow::Result<Self> {
        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users SET name = $1 WHERE id = $2
            RETURNING id, name
            "#,
            name,
            id
        )
        .fetch_one(pool)
        .await?;
        Ok(user)
    }

    pub async fn delete(pool: &sqlx::PgPool, id: i32) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM users WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;
        Ok(())
    }
    
    pub async fn list(pool: &sqlx::PgPool) -> anyhow::Result<Vec<Self>> {
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT id, name FROM users
            "#
        )
        .fetch_all(pool)
        .await?;
        Ok(users)
    }
}