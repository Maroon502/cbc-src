#![doc = include_str!("../README.md")]

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(feature = "cbcsolver")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));