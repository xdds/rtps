#![allow(dead_code)]
#![feature(custom_derive)]
#![feature(rustc_private)]

#[macro_use] extern crate log;

extern crate serde;
extern crate serde_json;

extern crate byteorder;

pub mod message;
pub mod discovery;
pub mod cdr;

pub use cdr::ser::*;
pub use message::*;
