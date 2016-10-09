#![feature(custom_derive)]
#![feature(type_ascription)]
#![feature(proc_macro)]

extern crate rtps;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate byteorder;

mod factories;

mod entity;
mod message;
mod submessage;