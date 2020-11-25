//! The web_midi error and result type

pub enum Error {}

pub type Result<T> = std::result::Result<T, Error>;
