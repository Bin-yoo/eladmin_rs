use serde::Serialize;

#[derive(Serialize)]
pub struct CodeResultVO {
    pub enabled: u8,
    pub img: Option<String>,
    pub uuid: Option<String>,
}