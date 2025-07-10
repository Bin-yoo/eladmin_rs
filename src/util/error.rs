use salvo::{async_trait, http::StatusCode, writing::Json, Depot, Request, Response, Writer};
use serde::Serialize;

#[derive(thiserror::Error, Debug, Serialize)]
pub enum AppError {
    #[error("Bad request: {0}")]
    BadRequest(String, String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    // 可以继续加其他类型的错误...
}

#[async_trait]
impl Writer for AppError {
    async fn write(self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        let (code, msg, data) = match &self {
            AppError::BadRequest(msg, data) => (StatusCode::BAD_REQUEST, msg.to_string(), data.to_string()),
            AppError::InternalServerError(data) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string(), data.to_string()),
        };

        res.status_code(code);
        res.render(Json(GlobalErrorResp {
            code: code.as_u16(),
            msg,
            data,
        }));
    }
}

impl From<anyhow::Error> for AppError {
    fn from(e: anyhow::Error) -> Self {
        AppError::InternalServerError(e.to_string())
    }
}

#[derive(Serialize)]
struct GlobalErrorResp {
    code: u16,
    msg: String,
    data: String,
}