use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// 用户实体，对应于数据库中的 `slzj_user` 表
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserEntity {
    /// 用户ID，自增主键
    pub user_id: i32,
    /// 用户名，可以为空
    pub user_name: Option<String>,
    /// 登录名，不能为空
    pub login_name: String,
    /// 密码，不能为空
    pub password: String,
    /// 行政区划ID，可以为空
    pub region_id: Option<i32>,
    /// 用户状态，1=正常（默认），0=禁用
    pub user_status: i8,
    /// 手机号，可以为空
    pub mobile: Option<String>,
    /// 备注，可以为空
    pub remark: Option<String>,
}

impl UserEntity {
    pub fn table_name() -> String {
        "slzj_user".to_string()
    }
}
