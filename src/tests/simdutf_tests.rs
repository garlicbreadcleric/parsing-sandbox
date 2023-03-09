extern crate test;
use test::bench::Bencher;

use super::test_data::*;

#[bench]
pub fn utf8_to_utf16_bench(b: &mut Bencher) {
  b.iter(|| {
    unsafe { simdutf::convert_arbitrary_utf8_to_utf16le(BENCHMARK_INPUT.as_bytes()) };
  });
}

#[bench]
pub fn utf16_length_from_utf8_bench(b: &mut Bencher) {
  b.iter(|| {
    unsafe { simdutf::count_utf16_from_utf8(BENCHMARK_INPUT.as_bytes()) };
  });
}
