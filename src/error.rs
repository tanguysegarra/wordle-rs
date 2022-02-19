#[derive(Debug)]
pub enum Error {
    IOError,
    RandFail,
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Error::IOError
    }
}
