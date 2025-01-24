use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};

use crate::common::error::AppResult;

//hash
pub fn argon_hash(content: impl AsRef<str>) -> AppResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon = Argon2::default();

    Ok(argon
        .hash_password(content.as_ref().as_bytes(), &salt)?
        .to_string())
}

//验证
pub fn argon_verify(content: impl AsRef<str>, hash: impl AsRef<str>) -> AppResult<()> {
    let hash_password = PasswordHash::new(hash.as_ref())?;
    Argon2::default().verify_password(content.as_ref().as_bytes(), &hash_password)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use fake::{Fake, Faker};

    use crate::{common::error::AppResult, utils::hash::argon_hash};

    use super::argon_verify;

    #[test]
    fn argon2_hash_test() -> AppResult<()> {
        let password: String = Faker.fake();
        let pass_hash = argon_hash(&password)?;
        println!("pass_hash:{}", pass_hash);
        assert_eq!(1, 2);
        Ok(())
    }

    #[test]
    fn argon_verify_test() -> AppResult<()> {
        let password: String = Faker.fake();
        let pass_hash = argon_hash(&password)?;
        let result = argon_verify(password, pass_hash);
        assert!(result.is_ok());
        Ok(())
    }
}
