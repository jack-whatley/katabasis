use tokio::sync::Semaphore;

pub mod fs;

/// The network semaphore, for controlling the number of concurrent downloads.
pub struct NetSemaphore(pub Semaphore);

/// The IO semaphore, for controlling the number of concurrent file operations.
pub struct IOSemaphore(pub Semaphore);
