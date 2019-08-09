use crate::error::*;
use std::io::{self, Write};

pub fn flush_stdout() -> ClimerResult {
    io::stdout().flush()?;
    Ok(())
}
