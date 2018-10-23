# raw-syscall-base
Low level system calls.

[![Latest version](https://img.shields.io/crates/v/raw-syscall-base.svg)](https://crates.io/crates/raw-syscall-base)
[![Documentation](https://docs.rs/raw-syscall-base/badge.svg)](https://docs.rs/raw-syscall-base)
[![Downloads](https://img.shields.io/crates/d/raw-syscall-base.svg)](https://crates.io/crates/raw-syscall-base)
[![License](https://img.shields.io/crates/l/raw-syscall-base.svg)](LICENSE.md)
[![Average time to resolve an issue](http://isitmaintained.com/badge/resolution/rvlzzr/raw-syscall-base.svg)](http://isitmaintained.com/project/rvlzzr/raw-syscall-base "Average time to resolve an issue")
[![Percentage of issues still open](http://isitmaintained.com/badge/open/rvlzzr/raw-syscall-base.svg)](http://isitmaintained.com/project/rvlzzr/raw-syscall-base "Percentage of issues still open")

## Usage
Add the following to your `Cargo.toml`:
```toml
raw-syscall-base = "0.8.2"
```

## Supported Platforms
* aarch64-linux
* arm-linux
* x86_64-freebsd
* x86_64-linux

## Purpose
This crate is limited to providing basic functionality necessary to perform system calls on the target platform.

All functions are marked unsafe, and no validation is done on arguments or return values.

All arguments and return values use the most basic possible types with results wrapped in a `Result`. For example all arguments are `usize` and return is `Result<usize, usize>` on `x86_64-linux`. All arguments must be converted to this basic type, and it's up to the caller to determine what the result represents.

The intention is to provide a minimal stable base with no unnecessary overhead on which to build a higher-level library.

## x86_64-linux Example
```rust
    use raw_syscall_base::{syscall, syscall_nr};
    
    // attempts to write "hello" to STDOUT
    pub unsafe fn hello() -> usize {
        syscall(1, &[1, b"hello" as *const u8 as usize, 5])
    }

    // exits the program with a success code 
    pub unsafe fn exit_success() -> ! {
        syscall_nr(231, &[0])
    }
```
