#![allow(dead_code)]
// #![feature(custom_derive)]
// #![feature(rustc_private)]

#![feature(plugin)]
#![plugin(clippy)]

//#[macro_use] extern crate log;
#[macro_use] extern crate bitflags;
extern crate serde;
extern crate serde_json;
extern crate byteorder;

pub mod entity;
pub mod message;
pub mod submessage;
pub mod cdr;
pub mod common_types;

//pub use cdr::ser::*;
pub use message::*;
pub use submessage::*;
pub use entity::*;