//! The `compiletest` binary. This will wire together the library to provide an executable.

// We must not naively use std `{e,}print{,ln}!` macros because they will panic on I/O errors like
// broken pipes, which have a terrible UX considering we are a test harness.
#![deny(clippy::print_stderr)]
#![deny(clippy::print_stdout)]
// Make sure things marked as `must_use` are used or explicitly allowed/suppressed. This is not a
// warning because we are the test harness and it's very bad if we cheap out on robust handling.
#![deny(unused_must_use)]

use compiletest::{Config, RawConfig};

fn main() {
    let _todo = RawConfig;
    let _todo2 = Config;

    todo!()
}
