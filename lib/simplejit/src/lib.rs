//! Top-level lib.rs for `cranelift_simplejit`.

#![deny(missing_docs, trivial_numeric_casts, unused_extern_crates)]
#![warn(unused_import_braces, unstable_features)]
#![cfg_attr(feature = "clippy", plugin(clippy(conf_file = "../../clippy.toml")))]
#![cfg_attr(feature = "cargo-clippy", allow(new_without_default, new_without_default_derive))]
#![cfg_attr(
    feature = "cargo-clippy",
    warn(
        float_arithmetic, mut_mut, nonminimal_bool, option_map_unwrap_or, option_map_unwrap_or_else,
        print_stdout, unicode_not_nfc, use_self
    )
)]

extern crate cranelift_codegen;
extern crate cranelift_module;
extern crate cranelift_native;
extern crate errno;
extern crate libc;
extern crate region;
extern crate target_lexicon;

#[cfg(target_os = "windows")]
extern crate winapi;

mod backend;
mod memory;

pub use backend::{SimpleJITBackend, SimpleJITBuilder};
