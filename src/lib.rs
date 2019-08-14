extern crate regex;
#[macro_use]
extern crate climer_derive;

mod settings;
#[macro_use]
pub mod macros;
pub mod error;
pub mod helpers;
pub mod time;
pub mod timer;

pub mod prelude {
    pub use super::error::{ClimerError, ClimerResult};
    pub use super::time::prelude::*;
    pub use super::timer::output::Output;
    pub use super::timer::{Timer, TimerBuilder, TimerState};
}

pub use prelude::*;
