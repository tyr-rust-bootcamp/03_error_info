use core::fmt;
use std::str::FromStr;

pub use error_code_derive::ToErrorInfo;

#[derive(Debug)]
pub struct ErrorInfo<T> {
    pub app_code: T,        // could be HTTP 400 bad request
    pub code: &'static str, // something like "01E739"
    pub client_msg: &'static str,
    pub server_msg: String,
}

pub trait ToErrorInfo {
    type T: FromStr;
    fn to_error_info(&self) -> Result<ErrorInfo<Self::T>, <Self::T as FromStr>::Err>;
}

impl<T> ErrorInfo<T>
where
    T: FromStr,
{
    pub fn try_new(
        app_code: &str,
        code: &'static str,
        client_msg: &'static str,
        server_msg: impl fmt::Display,
    ) -> Result<Self, T::Err> {
        Ok(Self {
            app_code: T::from_str(app_code)?,
            code,
            client_msg,
            server_msg: server_msg.to_string(),
        })
    }
}
