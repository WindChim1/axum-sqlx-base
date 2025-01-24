use tracing::debug;

use crate::{common::error::AppResult, utils::hash};

//hash密码
pub async fn hash(password: String) -> AppResult<String> {
    let jh = tokio::task::spawn_blocking(move || hash::argon_hash(password));
    jh.await?
}

pub async fn verify(password: String, hash_password: String) -> AppResult {
    let jh = tokio::task::spawn_blocking(move || hash::argon_verify(password, hash_password));
    if let Err(e) = jh.await? {
        debug!("The password is not correct:{}", e);
        Err(crate::common::error::AppErr::Argon2Err(
            "The password is no correct".to_string(),
        ))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use fake::{Fake, Faker};

    use crate::{
        common::error::AppResult,
        utils::passwor::{hash, verify},
    };

    #[tokio::test]
    async fn password_hash_test() -> AppResult<()> {
        let password: String = Faker.fake();
        let pass_hash = hash(password).await?;
        println!("pass_hash:{}", pass_hash);
        assert_eq!(1, 2);
        Ok(())
    }

    #[tokio::test]
    async fn password_verify_test() -> AppResult<()> {
        let password: String = Faker.fake();
        let pass_hash = hash(password.clone()).await?;
        let result = verify(password, pass_hash).await;
        assert!(result.is_ok());
        Ok(())
    }
}
