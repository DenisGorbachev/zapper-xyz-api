use errgonomic::handle;
use serde::Serialize;
use std::io::{Error as IoError, Write};
use thiserror::Error;

pub fn write_json_line(writer: &mut impl Write, value: &impl Serialize) -> Result<(), WriteJsonLineError> {
    use WriteJsonLineError::*;
    handle!(serde_json::to_writer(&mut *writer, value), ToWriterFailed);
    handle!(writeln!(writer), WriteNewlineFailed);
    Ok(())
}

#[derive(Error, Debug)]
pub enum WriteJsonLineError {
    #[error("failed to write JSON")]
    ToWriterFailed { source: serde_json::Error },
    #[error("failed to write newline")]
    WriteNewlineFailed { source: IoError },
}
