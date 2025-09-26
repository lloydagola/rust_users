use crate::models::{User, UserPayload};
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserRepository{
    pub pool: SqlitePool,
}

impl UserRepository{ 
    pub fn new(pool: SqlitePool) -> Self{
        Self{pool}
    }

    pub async fn list(&self) -> Vec<User>{
        sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&self.pool)
        .await
        .unwrap_or_default()
    }
    pub async fn create(&self, payload: UserPayload) -> User{
        let user = User{
            id: Uuid::new_v4().to_string(),
            name: payload.name,
            age: payload.age,
        };
        sqlx::query("INSERT INTO users (id, name, age) VALUES (?, ?, ?)")
        .bind(&user.id)
        .bind(&user.name)
        .bind(user.age)
        .execute(&self.pool)
        .await
        .unwrap();

        user
    }
    pub async fn get(&self, id: &str) -> Option<User> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .unwrap()
    }
    pub async fn update(&self, id: &str, payload: UserPayload) -> Option<User> {
        let result = sqlx::query("UPDATE users SET name = ?, age = ? WHERE id = ?")
            .bind(&payload.name)
            .bind(payload.age)
            .bind(id)
            .execute(&self.pool)
            .await
            .unwrap();
        if result.rows_affected() == 0 {
            None
        }
        else{
            self.get(id).await
        }
    }
    pub async fn delete(&self, id: &str) -> bool {
        let result = sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .unwrap();
        result.rows_affected()> 0
    }
}

