//! `compiletest` supports two layers of configuration, in order of precedence (last wins and has
//! highest precedence)
//!
//! 1. Environmental variables.
//! 2. Command-line arguments.
//!
//! This module is responsible for interfacing with bootstrap/environment. [`RawConfig`] is intended
//! to contain the raw cli options from bootstrap/environment without much validation. A separate
//! validation step will convert raw [`RawConfig`] into a validated [`Config`] for better
//! responsibility separation and diagnostics/handling.
//!
//! Configuration file is not supported.
//!
//! [`Config`]: crate::config::Config

// Remark(jieyoxuu): this cherry picks a few `compiletest` env vars and options just to play around
// with how they work together.

// TODO(jieyouxu): stub

/// Raw config options passed from bootstrap and the environment. Minimal-to-none validation.
///
/// This struct should not be consumed directly by later parts of compiletest; it should be
/// validated into a [`Config`].
///
/// [`Config`]: crate::config::Config
pub struct RawConfig;
