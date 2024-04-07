use axum::{
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use backtrace::Backtrace;
use error_code::ToErrorInfo;
use http::StatusCode;
use thiserror::Error;
use tokio::net::TcpListener;
use tracing::{info, warn};

#[allow(dead_code)]
#[derive(Debug, Error, ToErrorInfo)]
#[error_info(app_type = "StatusCode", prefix = "0A")]
enum AppError {
    #[error("Invalid param: {0}")]
    #[error_info(code = "IP", app_code = "400")]
    InvalidParam(String),

    #[error("Item {0} not found")]
    #[error_info(code = "NF", app_code = "404")]
    NotFound(String),

    #[error("Internal server error: {0}")]
    #[error_info(
        code = "ISE",
        app_code = "500",
        client_msg = "we had a server problem, please try again later"
    )]
    ServerError(String),

    #[error("Unknown error")]
    #[error_info(code = "UE", app_code = "500")]
    Unknown,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(index_handler));

    let addr = "0.0.0.0:8081";
    info!("Listening on http://{}", addr);
    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

async fn index_handler() -> Result<&'static str, AppError> {
    let bt = Backtrace::new();
    Err(AppError::ServerError(format!("{bt:?}")))
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let info = self.to_error_info();

        let status = info.app_code;

        if status.is_server_error() {
            warn!("{:?}", info);
        } else {
            info!("{:?}", info);
        }

        // use client-facing message
        Response::builder()
            .status(status)
            .body(info.to_string().into())
            .unwrap()
    }
}
