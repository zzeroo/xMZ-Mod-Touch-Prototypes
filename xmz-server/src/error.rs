use std::fmt;
use std::error::Error;
use std::io;


#[derive(Debug)]
pub enum ServerError {
    Io(io::Error),
    CouldNotStart,
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ServerError::Io(ref err) => err.fmt(f),
            ServerError::CouldNotStart => write!(f, "Server konnte nicht gestartet werden."),
        }
    }
}

impl Error for ServerError {
    fn description(&self) -> &str {
        match *self {
            ServerError::Io(ref err) => err.description(),
            ServerError::CouldNotStart => "Server konnte nicht gestartet werden",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ServerError::Io(ref err) => Some(err),
            ServerError::CouldNotStart => None,
        }
    }
}

impl From<io::Error> for ServerError {
    fn from(err: io::Error) -> ServerError {
        ServerError::Io(err)
    }
}
