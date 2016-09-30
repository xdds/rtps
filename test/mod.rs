#![feature(custom_derive)]
#![feature(type_ascription)]
#![feature(rustc_macro)]

extern crate rtps;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod factories;

mod entity;
mod message;
mod submessage;