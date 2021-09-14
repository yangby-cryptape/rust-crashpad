use thiserror::Error;

const HANDLER_MAN_URL: &str =
    "https://chromium.googlesource.com/crashpad/crashpad/+/HEAD/handler/crashpad_handler.md";

#[derive(Debug, Error)]
pub enum Error {
    #[error(
        "the exectable binary `{}` was not found since {}, more details can be found at {}",
        crashpad_sys::handler_name(),
        _0,
        HANDLER_MAN_URL
    )]
    Which(which::Error),
    #[error("handler error: {}", _0)]
    Handler(String),
    #[error("{}", _0)]
    Other(String),
}

pub type Result<T> = ::std::result::Result<T, Error>;
