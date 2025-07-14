use chrono::Utc;
use salvo::{async_trait, http::StatusCode, writing::Json, Depot, Request, Response, Writer};
use serde::Serialize;

#[derive(thiserror::Error, Debug, Serialize)]
pub enum AppError {
    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("Username not found")]
    UsernameNotFound,

    // 可以继续加其他类型的错误...
}

#[async_trait]
impl Writer for AppError {
    async fn write(self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        let code = match &self {
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::Forbidden => StatusCode::FORBIDDEN,
            AppError::UsernameNotFound => StatusCode::BAD_REQUEST
        };

        res.status_code(code);
        res.render(Json(GlobalErrorResp {
            status: code.as_u16(),
            message: format!("{}", &self),
            timestamp: chrono::Utc::now(),
        }));
    }
}

impl From<anyhow::Error> for AppError {
    fn from(e: anyhow::Error) -> Self {
        AppError::InternalServerError(e.to_string())
    }
}

impl From<salvo::http::ParseError> for AppError {
    fn from(e: salvo::http::ParseError) -> Self {
        AppError::BadRequest(format!("Request parse error: {}", e))
    }
}

impl From<rbs::Error> for AppError {
    fn from(e: rbs::Error) -> Self {
        AppError::InternalServerError(format!("[Database Error] {}", e))
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::BadRequest(format!("JSON parse error: {}", e))
    }
}

#[derive(Serialize)]
struct GlobalErrorResp {
    status: u16,
    message: String,
    timestamp: chrono::DateTime<Utc>
}