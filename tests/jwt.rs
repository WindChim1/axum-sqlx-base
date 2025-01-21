use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

#[derive(Debug)]
struct User {
    username: String,
    password: String,
}

impl User {
    fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.to_owned(),
            password: password.to_owned(),
        }
    }
    fn generate_jwt(&self) -> String {
        let header = Header::default();
        let secret = "my_secret_key";
        encode(
            &header,
            &self.username,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .unwrap()
    }

    fn verify_jwt(token: String) -> String {
        todo!()
    }
}

#[test]
fn jwt_test() {
    let token = User::new("wdc", "dev").generate_jwt();
    println!("token={}", token);
    assert_eq!(1, 2);
}
