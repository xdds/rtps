// TODO: for a good time, comment this line and let rust point out all the extra stuff
#![allow(dead_code)]

#![feature(proc_macro)]
#![feature(type_ascription)]
#![feature(rustc_attrs)]

// Clippy specific settings
#![feature(plugin)]
#![plugin(clippy)]
#![allow(len_without_is_empty)]

// Serde specific settings
extern crate serde;
#[macro_use] extern crate serde_derive;

//#[macro_use] extern crate log;
#[macro_use] extern crate bitflags;


extern crate byteorder;

// mod only exposed
pub mod cdr;
pub mod common_types;

// flattened expose
pub mod entity;
pub mod message;
pub mod submessage;

// glob expose
//pub use cdr::*;
pub use message::*;
pub use submessage::*;
pub use entity::*;