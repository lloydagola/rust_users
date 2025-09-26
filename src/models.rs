
use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct User{
    pub id: String,
    pub name: String,
    pub age: u32,
}

#[derive(Deserialize)]
pub struct UserPayload{
    pub name: String,
    pub age: u32,
}