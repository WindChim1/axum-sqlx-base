use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UserVo {
    pub login_name: String,
    pub password: String,
}
