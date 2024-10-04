use core::fmt;
use std::str::FromStr;

pub struct ErrorInfo<T> {
    pub app_code: T,
    pub code: &'static str,
    pub client_msg: &'static str,
    pub server_msg: String,
}

pub trait ToErrorInfo {
    type T: FromStr;
    fn to_error_info(&self) -> ErrorInfo<Self::T>;
}

impl<T> ErrorInfo<T>
where
    T: FromStr,
    <T as FromStr>::Err: fmt::Debug,
{
    pub fn new(
        app_code: &str,
        code: &'static str,
        client_msg: &'static str,
        server_msg: impl fmt::Display,
    ) -> Self {
        Self {
            app_code: T::from_str(app_code).expect("can not parse app code"),
            code,
            client_msg,
            server_msg: server_msg.to_string(),
        }
    }
}

impl<T> fmt::Display for ErrorInfo<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.client_msg)
    }
}

impl<T> fmt::Debug for ErrorInfo<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.client_msg)
    }
}
