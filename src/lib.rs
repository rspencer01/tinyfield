#![doc = include_str!("../README.md")]
#![no_std]
#![allow(non_upper_case_globals)]

pub mod prime_field;
pub mod prime_power_field;
pub mod fields;

pub use fields::*;
