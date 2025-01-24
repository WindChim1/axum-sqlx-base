use serde::{Deserialize, Serialize};

use crate::common::role::Role;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CustomerDto {
    pub user_id: i32,
    pub uasername: Option<String>,
    pub password: String,
    pub role: Role,
}

impl CustomerDto {
    pub fn builder() -> Self {
        Self::default()
    }
    pub fn user_id(mut self, user_id: i32) -> Self {
        self.user_id = user_id;
        self
    }

    pub fn username(mut self, username: String) -> Self {
        self.uasername = Some(username);
        self
    }
    pub fn password(mut self, password: String) -> Self {
        self.password = password;
        self
    }
    pub fn role(mut self, role: Role) -> Self {
        self.role = role;
        self
    }
    // pub fn build(self) -> Self {}
}
