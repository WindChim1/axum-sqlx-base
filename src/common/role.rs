use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Role {
    Admin(i32),
    User(i32),
    System(i32),
}

impl Default for Role {
    fn default() -> Self {
        Role::User(2)
    }
}
