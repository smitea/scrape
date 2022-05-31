use std::fmt::Display;

#[macro_export]
macro_rules! code {
    ($base : expr, $index : expr) => {{
        $base | ($index << 8)
    }};
}

#[macro_export]
macro_rules! from_code {
    ($fun: ident ,$variant: expr, $T:ty) => {
        pub fn $fun(err: $T) -> Self {
            Self {
                code: $variant,
                msg: err.to_string(),
            }
        }
    };
}

#[macro_export]
macro_rules! is_base_code {
    ($fun: ident , $base_code: expr) => {
        pub fn $fun(&self) -> bool {
            return self.get_base_code() == $base_code;
        }
    };
}

#[macro_export]
macro_rules! is_code {
    ($fun: ident , $($code: expr),*) => {
        pub fn $fun(&self) -> bool {
            $(if self.code == $code { return true})*
            return false;
        }
    };
}

const INTERNAL: i32 = code!(0x01, 0x00);
const INVALID: i32 = code!(0x02, 0x00);
const CHANNEL: i32 = code!(0x03, 0x00);
const IO: i32 = code!(0x04, 0x00);
const OTHER: i32 = code!(0x05, 0x00);
const OS: i32 = code!(0x08, 0x00);
const WEB3: i32 = code!(0x09, 0x00);

/// SQL 错误类型
const SQL: i32 = code!(0x0A, 0x00);
const SQL_CONNECTION_NUM_LIMIT: i32 = code!(SQL, 0x01);

const INVALID_TYPE: i32 = code!(INVALID, 0x01);
const INVALID_UTF8: i32 = code!(INVALID, 0x02);
const INVALID_PATH: i32 = code!(INVALID, 0x03);
const INVALID_INDEX: i32 = code!(INVALID, 0x04);
const INVALID_PARAM: i32 = code!(INVALID, 0x05);
const INVALID_NOT_SUPPORT: i32 = code!(INVALID, 0x06);
const INVALID_AUTH: i32 = code!(INVALID, 0x07);
const INVALID_DATABASE: i32 = code!(INVALID, 0x08);
const INVALID_URL: i32 = code!(INVALID, 0x09);
const INVALID_UTF16: i32 = code!(INVALID, 0x10);

const IO_NOTFOUND: i32 = code!(IO, 0x01);
const IO_PERMISSION_DENIED: i32 = code!(IO, 0x02);
const IO_CONNECTION_REFUSED: i32 = code!(IO, 0x03);
const IO_CONNECTION_RESET: i32 = code!(IO, 0x04);
const IO_CONNECTION_ABORTED: i32 = code!(IO, 0x05);
const IO_NOT_CONNECTED: i32 = code!(IO, 0x06);
const IO_ADDR_INUSE: i32 = code!(IO, 0x07);
const IO_ADDR_NOT_AVAILABLE: i32 = code!(IO, 0x08);
const IO_BROKEN_PIPE: i32 = code!(IO, 0x09);
const IO_ALREADY_EXISTS: i32 = code!(IO, 0x10);
const IO_INVALID_INPUT: i32 = code!(IO, 0x11);
const IO_INVALID_DATA: i32 = code!(IO, 0x12);
const IO_TIMED_OUT: i32 = code!(IO, 0x13);
const IO_WRITE_ZERO: i32 = code!(IO, 0x14);
const IO_UNEXPECTED_EOF: i32 = code!(IO, 0x16);
const IO_INTERRUPTED: i32 = code!(IO, 0x17);
const IO_WOULD_BLOCK: i32 = code!(IO, 0x18);
const IO_NOT_CONNECTED_HOST: i32 = code!(IO, 0x19);

const CHANNEL_SEND: i32 = code!(CHANNEL, 0x01);
const CHANNEL_RECV: i32 = code!(CHANNEL, 0x02);
const CHANNEL_CLOSE: i32 = code!(CHANNEL, 0x03);

const OS_SYSTEM: i32 = code!(OS, 0x01);

const WEB_CONTRACT: i32 = code!(WEB3, 0x01);

/// 错误信息(错误码和错误信息组成)
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Error {
    pub(crate) code: i32,
    pub(crate) msg: String,
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub fn new(code: i32, msg: &str) -> Error {
        Self {
            code,
            msg: msg.to_string(),
        }
    }

    pub fn get_code(&self) -> i32 {
        *(&self.code)
    }

    pub fn get_msg(&self) -> &str {
        &self.msg
    }

    pub fn decode(&self) -> (i32, i32) {
        let base = self.code >> 8;
        let code = self.code & 0xFF;
        return (base, code);
    }

    pub fn get_base_code(&self) -> i32 {
        return self.code >> 8;
    }

    from_code!(internal, INTERNAL, &str);
    from_code!(invalid, INVALID, &str);
    from_code!(invalid_type, INVALID_TYPE, &str);
    from_code!(invalid_utf8, INVALID_UTF8, &str);
    from_code!(invalid_utf16, INVALID_UTF16, &str);
    from_code!(invalid_url, INVALID_URL, &str);
    from_code!(invalid_path, INVALID_PATH, &str);
    from_code!(invalid_index, INVALID_INDEX, &str);
    from_code!(invalid_auth, INVALID_AUTH, &str);
    from_code!(invalid_database, INVALID_DATABASE, &str);
    from_code!(not_support, INVALID_NOT_SUPPORT, &str);
    from_code!(not_found, IO_NOTFOUND, &str);
    from_code!(permission_denied, IO_PERMISSION_DENIED, &str);
    from_code!(connection_refused, IO_CONNECTION_REFUSED, &str);
    from_code!(connection_reset, IO_CONNECTION_RESET, &str);
    from_code!(connection_aborted, IO_CONNECTION_ABORTED, &str);
    from_code!(not_connected, IO_NOT_CONNECTED, &str);
    from_code!(addr_inuse, IO_ADDR_INUSE, &str);
    from_code!(addr_not_available, IO_ADDR_NOT_AVAILABLE, &str);
    from_code!(io, IO, &str);
    from_code!(broken_pipe, IO_BROKEN_PIPE, &str);
    from_code!(already_exists, IO_ALREADY_EXISTS, &str);
    from_code!(invalid_input, IO_INVALID_INPUT, &str);
    from_code!(invalid_param, INVALID_PARAM, &str);
    from_code!(invalid_data, IO_INVALID_DATA, &str);
    from_code!(timeout, IO_TIMED_OUT, &str);
    from_code!(write_zero, IO_WRITE_ZERO, &str);
    from_code!(unexpected_eof, IO_UNEXPECTED_EOF, &str);
    from_code!(interrupted, IO_INTERRUPTED, &str);
    from_code!(would_block, IO_WOULD_BLOCK, &str);
    from_code!(not_connected_host, IO_NOT_CONNECTED_HOST, &str);
    from_code!(other, OTHER, &str);
    from_code!(channel_rev, CHANNEL_RECV, &str);
    from_code!(channel_send, CHANNEL_SEND, &str);
    from_code!(channel_close, CHANNEL_CLOSE, &str);
    from_code!(sql, SQL, &str);
    from_code!(connection_num_limit, SQL_CONNECTION_NUM_LIMIT, &str);

    is_base_code!(is_sql_err, SQL);
    is_base_code!(is_io_err, IO);
    is_base_code!(is_invalid_err, INVALID);
    is_base_code!(is_other_err, OTHER);
    is_base_code!(is_os_err, OS);

    is_code!(is_invalid_database_err, INVALID_DATABASE);
    is_code!(is_invalid_auth_err, INVALID_AUTH);
    is_code!(is_timeout_err, IO_TIMED_OUT);
    is_code!(is_addr_inuse, IO_ADDR_INUSE, IO_ADDR_NOT_AVAILABLE);
    is_code!(is_not_connected_host, IO_NOT_CONNECTED_HOST);
    is_code!(
        is_connection_err,
        IO_CONNECTION_ABORTED,
        IO_CONNECTION_REFUSED,
        IO_CONNECTION_RESET,
        IO_INTERRUPTED,
        IO_NOT_CONNECTED,
        IO_UNEXPECTED_EOF,
        IO_WRITE_ZERO,
        IO_WOULD_BLOCK
    );
    is_code!(
        is_channel_err,
        CHANNEL_SEND,
        CHANNEL_CLOSE,
        CHANNEL_RECV,
        CHANNEL
    );
    is_code!(is_connection_num_limit, SQL_CONNECTION_NUM_LIMIT);
}

macro_rules! from_error {
    ($variant: expr, $T:ty) => {
        impl From<$T> for Error {
            fn from(err: $T) -> Self {
                Error::new($variant, &err.to_string())
            }
        }
    };
}

from_error!(CHANNEL_RECV, tokio::sync::mpsc::error::RecvError);
impl<T> From<tokio::sync::mpsc::error::SendError<T>> for Error {
    fn from(err: tokio::sync::mpsc::error::SendError<T>) -> Self {
        Error::new(CHANNEL_SEND, &err.to_string())
    }
}

from_error!(INVALID_UTF8, std::string::FromUtf8Error);
from_error!(INVALID_UTF8, std::str::Utf8Error);
from_error!(INVALID_PATH, std::convert::Infallible);
from_error!(INVALID_TYPE, std::num::ParseIntError);
from_error!(INVALID_TYPE, std::num::ParseFloatError);
from_error!(IO_TIMED_OUT, tokio::time::error::Elapsed);
from_error!(OS_SYSTEM, std::time::SystemTimeError);

from_error!(WEB3, web3::Error);
from_error!(WEB_CONTRACT, web3::contract::Error);

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        let msg = err.to_string();
        match err.kind() {
            std::io::ErrorKind::NotFound => Error::not_found(&msg),
            std::io::ErrorKind::PermissionDenied => Error::permission_denied(&msg),
            std::io::ErrorKind::ConnectionRefused => Error::connection_refused(&msg),
            std::io::ErrorKind::ConnectionReset => Error::connection_reset(&msg),
            std::io::ErrorKind::ConnectionAborted => Error::connection_aborted(&msg),
            std::io::ErrorKind::NotConnected => Error::not_connected(&msg),
            std::io::ErrorKind::AddrInUse => Error::addr_inuse(&msg),
            std::io::ErrorKind::AddrNotAvailable => Error::addr_not_available(&msg),
            std::io::ErrorKind::BrokenPipe => Error::broken_pipe(&msg),
            std::io::ErrorKind::AlreadyExists => Error::already_exists(&msg),
            std::io::ErrorKind::WouldBlock => Error::would_block(&msg),
            std::io::ErrorKind::InvalidInput => Error::invalid_input(&msg),
            std::io::ErrorKind::InvalidData => Error::invalid_data(&msg),
            std::io::ErrorKind::TimedOut => Error::timeout(&msg),
            std::io::ErrorKind::WriteZero => Error::write_zero(&msg),
            std::io::ErrorKind::Interrupted => Error::interrupted(&msg),
            std::io::ErrorKind::UnexpectedEof => Error::unexpected_eof(&msg),
            _ => Error::io(&msg),
        }
    }
}

unsafe impl Send for Error {}

unsafe impl Sync for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} : {}", self.code, self.msg)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.msg
    }
}

#[test]
fn test() {
    let err = Error::new(6150, "");
    println!("err - {}", err);

    println!("INVALID_TYPE|{}", INVALID_TYPE);
    println!("INVALID_UTF8|{}", INVALID_UTF8);
    println!("INVALID_PATH|{}", INVALID_PATH);
    println!("INVALID_INDEX|{}", INVALID_INDEX);
    println!("INVALID_PARAM|{}", INVALID_PARAM);
    println!("INVALID_NOT_SUPPORT|{}", INVALID_NOT_SUPPORT);
    println!("INVALID_AUTH|{}", INVALID_AUTH);
    println!("INVALID_DATABASE|{}", INVALID_DATABASE);
    println!("INVALID_URL|{}", INVALID_URL);

    println!("IO_NOTFOUND|{}", IO_NOTFOUND);
    println!("IO_PERMISSION_DENIED|{}", IO_PERMISSION_DENIED);
    println!("IO_CONNECTION_REFUSED|{}", IO_CONNECTION_REFUSED);
    println!("IO_CONNECTION_RESET|{}", IO_CONNECTION_RESET);
    println!("IO_CONNECTION_ABORTED|{}", IO_CONNECTION_ABORTED);
    println!("IO_NOT_CONNECTED|{}", IO_NOT_CONNECTED);
    println!("IO_ADDR_INUSE|{}", IO_ADDR_INUSE);
    println!("IO_ADDR_NOT_AVAILABLE|{}", IO_ADDR_NOT_AVAILABLE);
    println!("IO_BROKEN_PIPE|{}", IO_BROKEN_PIPE);
    println!("IO_ALREADY_EXISTS|{}", IO_ALREADY_EXISTS);
    println!("IO_INVALID_INPUT|{}", IO_INVALID_INPUT);
    println!("IO_INVALID_DATA|{}", IO_INVALID_DATA);
    println!("IO_TIMED_OUT|{}", IO_TIMED_OUT);
    println!("IO_WRITE_ZERO|{}", IO_WRITE_ZERO);
    println!("IO_UNEXPECTED_EOF|{}", IO_UNEXPECTED_EOF);
    println!("IO_INTERRUPTED|{}", IO_INTERRUPTED);
    println!("IO_WOULD_BLOCK|{}", IO_WOULD_BLOCK);
    println!("IO_NOT_CONNECTED_HOST|{}", IO_NOT_CONNECTED_HOST);

    println!("CHANNEL_SEND|{}", CHANNEL_SEND);
    println!("CHANNEL_RECV|{}", CHANNEL_RECV);
    println!("CHANNEL_CLOSE|{}", CHANNEL_CLOSE);

    println!("OS_SYSTEM|{}", OS_SYSTEM);

    println!("SQL|{}", SQL);
    println!("SQL_CONNECTION_NUM_LIMIT|{}", SQL_CONNECTION_NUM_LIMIT);

    println!("WEB3|{}", WEB3);
    println!("WEB_CONTRACT|{}", WEB_CONTRACT);
}
