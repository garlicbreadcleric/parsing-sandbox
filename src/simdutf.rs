mod ffi {
  #[link(name = "simdutf_wrapper")]
  extern "C" {
    pub fn count_utf8(input: *const u8, length: usize) -> usize;
    pub fn utf16_length_from_utf8(input: *const u8, length: usize) -> usize;
    pub fn convert_valid_utf8_to_utf16le(input: *const u8, length: usize, utf16_buffer: *mut u16) -> usize;
  }
}

pub fn utf8_to_utf16(input: &[u8]) -> Vec<u16> {
  let length = unsafe { ffi::utf16_length_from_utf8(input.as_ptr(), input.len()) };
  let mut buf = Vec::with_capacity(length);
  unsafe { ffi::convert_valid_utf8_to_utf16le(input.as_ptr(), length, buf.as_mut_slice().as_mut_ptr()) };
  buf
}

pub fn count_utf8(input: &[u8]) -> usize {
  unsafe { ffi::count_utf8(input.as_ptr(), input.len()) }
}

#[cfg(test)]
mod tests {
  extern crate test;
  use test::bench::Bencher;

  use super::*;
  use crate::test_data::*;

  #[bench]
  pub fn utf8_to_utf16_bench(b: &mut Bencher) {
    b.iter(|| {
      utf8_to_utf16(BENCHMARK_INPUT.as_bytes());
    });
  }
}
