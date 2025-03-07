#[derive(Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

impl Credentials {
    pub fn new(username:&str, password:&str)->Option<Credentials>{
        Some(Credentials{username:username.to_string(),password:password.to_string()})
    }
}
