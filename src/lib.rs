#![doc = include_str!("../README.md")]

#[cfg(feature = "cbcsolver")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));