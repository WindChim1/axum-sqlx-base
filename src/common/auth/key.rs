use std::{
    fmt::{Debug, Display},
    time::Duration,
};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use uuid::Uuid;

use crate::common::constant::EXPIRE_SESSION_CODE_SECS;

pub trait Key: Debug + Display {
    type Value: Serialize + DeserializeOwned + Debug;
    const EXPIR_TIME: Duration;

    fn expir(&self) -> Duration {
        Self::EXPIR_TIME
    }
    fn value(&self) -> Self::Value;
}

#[doc = "session key"]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct SessionKey {
    pub user_id: String,
}

impl Key for SessionKey {
    type Value = Uuid;

    const EXPIR_TIME: Duration = EXPIRE_SESSION_CODE_SECS;

    fn value(&self) -> Self::Value {
        Uuid::new_v4()
    }
}
impl Display for SessionKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SESSION_KEY:{}", self.user_id)
    }
}
