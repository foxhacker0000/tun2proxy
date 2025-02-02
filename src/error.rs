#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("std::ffi::NulError {0:?}")]
    Nul(#[from] std::ffi::NulError),

    #[error("ctrlc::Error {0:?}")]
    InterruptHandler(#[from] ctrlc::Error),

    #[error("std::io::Error {0}")]
    Io(#[from] std::io::Error),

    #[error("std::net::AddrParseError {0}")]
    AddrParse(#[from] std::net::AddrParseError),

    #[error("smoltcp::iface::RouteTableFull {0:?}")]
    RouteTableFull(#[from] smoltcp::iface::RouteTableFull),

    #[error("smoltcp::socket::tcp::RecvError {0:?}")]
    Recv(#[from] smoltcp::socket::tcp::RecvError),

    #[error("smoltcp::socket::tcp::ListenError {0:?}")]
    Listen(#[from] smoltcp::socket::tcp::ListenError),

    #[error("smoltcp::socket::udp::BindError {0:?}")]
    Bind(#[from] smoltcp::socket::udp::BindError),

    #[error("smoltcp::socket::tcp::SendError {0:?}")]
    Send(#[from] smoltcp::socket::tcp::SendError),

    #[error("std::str::Utf8Error {0:?}")]
    Utf8(#[from] std::str::Utf8Error),

    #[cfg(target_os = "android")]
    #[error("jni::errors::Error {0:?}")]
    Jni(#[from] jni::errors::Error),

    #[error("&str {0}")]
    Str(String),

    #[error("String {0}")]
    String(String),

    #[error("&String {0}")]
    RefString(String),

    #[error("nix::errno::Errno {0:?}")]
    OSError(#[from] nix::errno::Errno),

    #[error("std::num::ParseIntError {0:?}")]
    IntParseError(#[from] std::num::ParseIntError),
}

impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Self::Str(err.to_string())
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Self::String(err)
    }
}

impl From<&String> for Error {
    fn from(err: &String) -> Self {
        Self::RefString(err.to_string())
    }
}
