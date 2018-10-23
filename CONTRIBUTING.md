# Contributing
The following guidelines should be followed for any contributions.

## Code
* This crate is to have no external dependencies.
* This crate should only provide generic functions to perform system calls. Named functions for individual system calls belong in higher-level crates such as `raw-syscall-enumerated`, not here.
* New platforms should match the `x86_64-linux` implementation as closely as possible.
* Code should be well documented and pass `cargo check` and `cargo clippy` with all/pedantic warnings enabled. Warnings should be disabled per-function as needed.

## Naming
* Identifiers should represent the register or location on the stack or such where arguments are placed.
* Identifiers matching reserved words should use raw identifier format (eg `type` should become `r#type`)

## License
* Any contributed code will be made available under the MIT-0/0BSD license used by this project.
* Do not contribute if this license is not acceptable to you.
* Do not include any code not compatible with this license in your contribution.

## Attribution
* If you make any contributions, add yourself to `CONTRIBUTORS.md` if you like.
* If you make significant contributions (such as adding support for a new platform) and plan on maintaining them, add yourself as an author to `Cargo.toml` as well if you like.
* Note that our license does not require any form of attribution and as such we reserve the right to remove your name from these or any other locations at any time for any or no reason.
