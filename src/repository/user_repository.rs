use std::sync::Arc;

use tracing_log::log::info;

use crate::{
    common::{
        client::database::DatabaseClient,
        error::{AppErr, AppResult},
    },
    dto::customer_dto::CustomerDto,
    entity::user_entity::UserEntity,
};

pub struct UserRepository {
    db: Arc<DatabaseClient>,
}

impl UserRepository {
    pub fn new(db: Arc<DatabaseClient>) -> Self {
        Self { db }
    }
    fn table_name(&self) -> String {
        UserEntity::table_name()
    }

    pub async fn find_by_login_name(&self, login_nam: &str) -> AppResult<CustomerDto> {
        info!("find user by login_nam :{}", login_nam);
        let sql = format!("select * from {} where login_name = ? ", self.table_name(),);
        let result = sqlx::query_as::<_, UserEntity>(&sql)
            .bind(login_nam)
            .fetch_optional(&*self.db)
            .await?;
        match result {
            Some(user) => {
                let mut customer = CustomerDto::builder()
                    .user_id(user.user_id)
                    .password(user.password);
                if let Some(username) = user.user_name {
                    customer = customer.username(username);
                };
                Ok(customer)
            }
            None => Err(AppErr::UserNotFoundErr(format!("{} 用户不存在", login_nam))),
        }
    }
}

#[tokio::test]
async fn query_user_test() -> AppResult<()> {
    use crate::common::client::ClientBuilder;
    use crate::common::config::env;
    use crate::common::config::AppConfig;

    let config = AppConfig::inint_config(env::get_env_source("APP")).unwrap();

    let client = DatabaseClient::build_from_config(&config).await?;
    let user_repository = UserRepository::new(Arc::new(client));
    let user = user_repository.find_by_login_name("adminSC").await?;
    println!("user :{:?}", user);
    assert_eq!(1, 2);
    Ok(())
}
