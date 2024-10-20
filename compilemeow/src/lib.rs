//! `compiletest` is the main test harness for running test suites under `tests/`. It supports both
//! `rustc` and `rustdoc` test suites. This is the library portion and "guts" of the test harness.
//! Try to keep the binary as minimal as possible.
//!
//! This top-level crate doc comment is intended to provide a high-level overview of the test
//! harness as well as some context and considerations. Please improve both this crate-level doc
//! comment as well as module-level doc comments as suitable.
//!
//! <div class="warning">
//! This module is the main orchestrator. Keep any specific implementation details out of `lib.rs`!
//! This module is the public API, handling module declaration and re-exports.
//! </div>
//!
//! ------------------------------------------------------------------------------------------------
//! # Core phases
//!
//! ## Configuration collection
//!
//! TODO
//!
//! ## Configuration validation
//!
//! TODO
//!
//! ## Test discovery
//!
//! TODO
//!
//! ## Directive collection
//!
//! TODO
//!
//! ## Directive validation
//!
//! ### Naive recognition
//!
//! TODO
//!
//! ### Early validation into structured representation
//!
//! TODO
//!
//! ### Late validation and test config assembly
//!
//! TODO
//!
//! ## Libtest preparation and test package assembly
//!
//! TODO
//!
//! ## Libtest execution
//!
//! TODO
//!
//! ## Report generation / metrics generation
//!
//! TODO
//!
//! ------------------------------------------------------------------------------------------------
//! # Considerations for cross-cutting concerns
//!
//! ## Error handling and diagnostics
//!
//! TODO
//!
//! - Have proper user-facing error levels: warn vs error vs info.
//! - Can we use some color when this is available, i.e. in a supported tty (heuristically)?
//!
//! ## Metrics collection and generation
//!
//! TODO
//!
//! ------------------------------------------------------------------------------------------------
//! # Testing
//!
//! ## Unit testing
//!
//! TODO
//!
//! ## Internal integration testing
//!
//! TODO
//!
//! ## External integration testing
//!
//! TODO
//!
//! ------------------------------------------------------------------------------------------------
//! # Logging
//!
//! ## `compiletest` developer-facing logging
//!
//! - Probably with `tracing`, but don't rely on `tracing` naively for user-facing diagnostics.
//! - Explicitly set target name as `compiletest`.
//!
//! ## CI-facing logging
//!
//! - Consider looking at how `opt-dist` does it with GH CI log groups.
//!
//! ------------------------------------------------------------------------------------------------
//! # Documentation
//!
//! ## `compiletest` self docs
//!
//! TODO
//!
//! ## Test modes and test suites
//!
//! TODO
//!
//! QUESTION(jieyouxu): do we really need the "test mode" abstraction?
//!
//! ## Directives
//!
//! TODO
//!
//! QUESTION(jieyouxu): how can we make sure each directive contain these vital info on
//! `nightly-rustc` docs:
//!
//! - Directive name
//! - Directive flavor: name-only, name-value, name-special?
//! - Supported test suites
//! - Mutually incompatible with...?
//! - Quirks, warnings and remarks
//! - Interactions with e.g. `compile-flags` or `exec-env` or such.
//!
//! Can we enforce that each directive have these entries, maybe something like `define_lint!`?
//!
//! ------------------------------------------------------------------------------------------------
//! # Other remarks
//!
//! ## Discipline of information sharing and data dependencies
//!
//! - For each phase, require the bare minimum of info. Don't have a global ctxt that contains *all*
//!   the info, but only probably a diagnostics handler. The idea here is to make it as easy to test
//!   in isolated parts as possible. E.g. if we just pass a diagnostics handler and not a full blown
//!   global ctxt, then we can easily stub out the diagnostics handler then check behavior of a
//!   function/component with respect to inputs controlled and provided by the test. See testability
//!   section below as well.
//! - For the above purpose, don't be afraid to introduce phase-local context structs that have a
//!   shorter lifetime (roughly phase-local) than the entire test session.
//!
//! ## Testability and reproducibility is critical
//!
//! - The test harness itself must be reliable. Afterall, if the test harness is not reliable, how
//!   can we claim rustc/rustdoc and their associated tests are reliable?
//!     - To aid with this, we need to make sure `compiletest` itself has a high self-coverage,
//!       through a multitude of test granularity (i.e. unit tests or integration tests are not
//!       sufficient alone, these are needed to complement each other for better test coverage).
//! - Spurious failures are the worst kind of failures.
//! - We need to properly account for environments, especially surrounding properly supporting
//!   cross-compilation scenarios and remote-test-client.
//!
//! ## Centralize stderr/stdout handling
//!
//! - Do not use `print{,ln}!` or `eprint{,ln}!` for production, use `write{,ln}` with explicit
//!   error handling (omitting the error is one of the possible valid strategies). The std macros do
//!   not handle I/O errors and will just panic, which is not great UX (e.g. broken pipes).
//! - Probably provide centralized output helpers so it's easier to centralize / enforce consistent
//!   style. Probably ban `print{,ln}!` and `eprint{,ln}!` with the clippy lints.
//!
//! ## Performance is important
//!
//! We should have self-profiling (connected to metrics) so we can identify which parts of the
//! `compiletest` logic take up the most time. This should make it easier for contributors if they
//! want to optimize/tune specific phases.
//!
//! ## Decouple from libtest specifics, don't design libtest-central logic
//!
//! Don't get held hostage by how libtest does things. Instead, consider libtest as a test runner
//! "backend" or plugin, keep this part flexible so we can swap out to other thing like
//! `libtest-next` or `ui_test` in the future should we have a viable alternative.
//!
//! ------------------------------------------------------------------------------------------------
//! # Useful references and resources
//!
//! TODO

// Helps to catch refactors and renaming breaking references.
#![deny(rustdoc::broken_intra_doc_links)]
// Use <link> instead and not bare URLs, or use proper markdown links.
#![deny(rustdoc::bare_urls)]
// We are a rust-lang/rust internal tool, unpublished.
#![allow(rustdoc::private_intra_doc_links)]
// Make sure things marked as `must_use` are used or explicitly allowed/suppressed. This is not a
// warning because we are the test harness and it's very bad if we cheap out on robust handling.
#![deny(unused_must_use)]
// Make sure things are documented. We can't cheap out on test harness docs!
#![warn(missing_docs)]
// We must not naively use std `{e,}print{,ln}!` macros because they will panic on I/O errors like
// broken pipes, which have a terrible UX considering we are a test harness.
#![deny(clippy::print_stderr)]
#![deny(clippy::print_stdout)]

// Warning: do not make everything `pub`-by-default, instead use the lowest visibility as needed to
// make it easier to tell what are implementation details specific to modules.

pub mod cli;
pub mod config;

// Documentation modules, inspired by clap!
pub mod __faq;
pub mod __tutorials;

// Remark: use `self::` path prefixes here so rustfmt has better grouping.

pub use self::cli::RawConfig;
pub use self::config::Config;
