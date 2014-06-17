
#![crate_type = "lib"]
#![crate_id="freetype#0.0.1"]
#![desc = "Rust bindings for FreeType, based on version 2.5.2"]
#![license = "MIT"]

#![feature(globs)]

extern crate libc;

pub use library::Library;
pub use face::Face;

pub mod ffi;
pub mod library;
pub mod face;

