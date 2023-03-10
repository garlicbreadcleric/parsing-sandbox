#![feature(test)]
#![feature(portable_simd)]

pub mod parsers_char;
pub mod parsers_utf16;
pub mod types;
pub mod utf8;

#[cfg(test)]
mod tests;
