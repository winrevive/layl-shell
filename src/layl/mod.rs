pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub(crate) mod cmdchecker;
pub(crate) mod color;
pub(crate) mod file_interpret;
pub(crate) mod init;
pub(crate) mod io;
pub(crate) mod power;
pub(crate) mod processmgmt;
pub(crate) mod reg;
pub(crate) mod shell;
pub(crate) mod utils;
