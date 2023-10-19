use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BackendError {
    NotFound,
    Unknown(String),
}

pub trait Backend<T> {
    fn scan(&mut self) -> Result<Vec<u8>, BackendError>;
    fn connect(&mut self, device: T) -> Result<(), BackendError>;
    fn try_read(&mut self, timeout: Duration) -> Result<Option<Vec<u8>>, BackendError>;
    fn write(&mut self, bytes: &[u8]) -> Result<usize, BackendError>;
    fn close(&mut self);
}
