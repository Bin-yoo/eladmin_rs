use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthUserDTO {
    pub username: String,
    pub password: String,
    pub uuid: String,
    pub code: Option<String>,
}