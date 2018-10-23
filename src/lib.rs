//! # raw-syscall-base
//!
//! This crate is limited to providing basic functionality necessary to perform
//! system calls on the target platform.
//!
//! All functions are marked unsafe, and no validation is done on arguments or
//! return values.
//!
//! All arguments and return values use the most basic possible types, for
//! example everything is `usize` on `x86_64-linux`. All arguments must be
//! converted to this type, and it's up to the caller to determine whether the
//! result represents a pointer or error code or whatever.
//!
//! The intention is to provide a minimal stable base with no unnecessary
//! overhead on which to build a higher-level library.
//!
#![no_std]
#![no_implicit_prelude]
#![allow(unsafe_code)]
#![allow(unstable_features)]
#![feature(asm)]
#![cfg_attr(feature = "cargo-clippy", deny(clippy::all, clippy::pedantic, warnings))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::inline_always, clippy::similar_names))]

#[cfg(all(target_arch = "aarch64", target_os = "linux"))]
#[path = "aarch64-linux.rs"]
mod platform;

#[cfg(all(target_arch = "arm", target_os = "linux"))]
#[path = "arm-linux.rs"]
mod platform;

#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
#[path = "x86_64-linux.rs"]
mod platform;

#[cfg(all(target_arch = "x86_64", target_os = "freebsd"))]
#[path = "x86_64-freebsd.rs"]
mod platform;

pub use self::platform::*;
