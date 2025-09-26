use crate::models::{User, UserPayload};
use crate::repository::UserRepository;

#[derive(Clone)]
pub struct UserService{
    repo: UserRepository,
}

impl UserService{
    pub fn new(repo: UserRepository) -> Self {
        Self {repo}
    }

    pub async fn list_users(&self) -> Vec<User> {
        self.repo.list().await
    }

    pub async fn create_user(&self, payload: UserPayload) -> User {
        self.repo.create(payload).await
    }

    pub async fn get_user(&self, id: &str) -> Option<User> {
        self.repo.get(id).await
    }

    pub async fn update_user(&self, id: &str, payload: UserPayload) -> Option<User> {
        self.repo.update(id, payload).await
    }

    pub async fn delete_user(&self, id: &str) -> bool {
        self.repo.delete(id).await
    }
}
