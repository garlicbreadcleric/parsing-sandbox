#![feature(test)]
#![feature(portable_simd)]

pub mod parsers;
pub mod simdutf;
pub mod types;
pub mod utf8;

#[cfg(test)]
pub(crate) mod test_data;
