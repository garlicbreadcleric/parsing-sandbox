use std::arch::x86_64::*;
use std::simd::{u8x16, Simd, SimdPartialEq};

#[cfg(test)]
enum Vectorization {
  Intel128,
  Intel256,
  Portable128,
}

#[cfg(test)]
fn count_utf8_characters(bytes: &[u8], vectorization: Option<Vectorization>) -> usize {
  let mut i = 0;
  let mut count = 0;
  match vectorization {
    Some(Vectorization::Intel128) => {
      while i + 15 < bytes.len() {
        let v = unsafe { _mm_loadu_si128((bytes[i..].as_ptr()).cast()) };
        count += count_utf8_characters_v128(v);
        i += 16;
      }
    }
    Some(Vectorization::Intel256) => {
      while i + 31 < bytes.len() {
        let v = unsafe { _mm256_loadu_si256((bytes[i..].as_ptr()).cast()) };
        count += count_utf8_characters_v256(v);
        i += 32;
      }
    }
    Some(Vectorization::Portable128) => {
      while i + 15 < bytes.len() {
        let v = u8x16::from_slice(&bytes[i..]);
        count += count_utf8_characters_v128_portable(v);
        i += 16;
      }
    }
    None => {}
  }

  if i < bytes.len() {
    count += count_utf8_characters_scalar(&bytes[i..]);
  }
  count
}

pub fn count_utf8_characters_scalar(bytes: &[u8]) -> usize {
  bytes.iter().filter(|&&byte| !is_continuation_byte(byte)).count()
}

#[inline]
pub fn count_utf8_characters_v128(v: __m128i) -> usize {
  let cmp_result: Simd<u8, 16> =
    unsafe { _mm_cmpeq_epi8(_mm_and_si128(v, _mm_set1_epi8(0b1100_0000u8 as i8)), _mm_set1_epi8(0b1000_0000u8 as i8)) }
      .into();
  let continuation_bytes = cmp_result.as_array().iter().filter(|&&c| c == 255).count();

  16 - continuation_bytes
}

#[inline]
pub fn count_utf8_characters_v128_portable(v: u8x16) -> usize {
  let cmp_result = (v & u8x16::splat(0b1100_0000)).simd_eq(u8x16::splat(0b1000_0000));
  cmp_result.to_array().iter().filter(|&&c| !c).count()
}

#[inline]
pub fn count_utf8_characters_v256(v: __m256i) -> usize {
  let cmp_result: Simd<u8, 32> = unsafe {
    _mm256_cmpeq_epi8(_mm256_and_si256(v, _mm256_set1_epi8(0b1100_0000u8 as i8)), _mm256_set1_epi8(0b1000_0000u8 as i8))
  }
  .into();
  let continuation_bytes = cmp_result.as_array().iter().filter(|&&c| c == 255).count();

  32 - continuation_bytes
}

#[inline]
pub fn is_continuation_byte(byte: u8) -> bool {
  (byte as i8) < -64
}

#[inline]
pub fn get_character_width(starting_byte: u8) -> usize {
  if starting_byte < 0b1100_0000 {
    1
  } else if starting_byte < 0b1110_0000 {
    2
  } else if starting_byte < 0b1111_0000 {
    3
  } else {
    4
  }
}

#[cfg(test)]
mod tests {
  extern crate test;
  use test::bench::Bencher;

  use proptest::prelude::*;

  use super::*;
  use crate::tests::test_data::*;

  proptest! {
    #[test]
    fn count_characters_property_test(s in "\\PC*") {
      let c1 = count_utf8_characters(s.as_str().as_bytes(), Some(Vectorization::Intel128));
      let c2 = count_utf8_characters(s.as_str().as_bytes(), Some(Vectorization::Intel256));
      let c3 = count_utf8_characters(s.as_str().as_bytes(), Some(Vectorization::Portable128));
      let c4 = count_utf8_characters_scalar(s.as_str().as_bytes());
      let c5 = s.chars().count();

      assert_eq!(c1, c2, "c1 == c2");
      assert_eq!(c2, c3, "c2 == c3");
      assert_eq!(c3, c4, "c3 == c4");
      assert_eq!(c4, c5, "c4 == c5");
    }
  }

  #[test]
  pub fn count_characters_test() {
    let bytes1 = SHORT_ASCII_INPUT.as_bytes();
    let bytes2 = SHORT_UNICODE_INPUT.as_bytes();

    assert_eq!(count_utf8_characters(bytes1, Some(Vectorization::Intel128)), SHORT_ASCII_INPUT.chars().count());
    assert_eq!(count_utf8_characters(bytes2, Some(Vectorization::Intel128)), SHORT_UNICODE_INPUT.chars().count());

    assert_eq!(count_utf8_characters(bytes1, Some(Vectorization::Intel256)), SHORT_ASCII_INPUT.chars().count());
    assert_eq!(count_utf8_characters(bytes2, Some(Vectorization::Intel256)), SHORT_UNICODE_INPUT.chars().count());

    assert_eq!(count_utf8_characters(bytes1, Some(Vectorization::Portable128)), SHORT_ASCII_INPUT.chars().count());
    assert_eq!(count_utf8_characters(bytes2, Some(Vectorization::Portable128)), SHORT_UNICODE_INPUT.chars().count());

    assert_eq!(count_utf8_characters_scalar(bytes1), SHORT_ASCII_INPUT.chars().count());
    assert_eq!(count_utf8_characters_scalar(bytes2), SHORT_UNICODE_INPUT.chars().count());

    {
      let mut count1 = 0;
      let mut i = 0;

      while i < bytes1.len() {
        let right_bound = (i + 16).min(bytes1.len());
        count1 += count_utf8_characters(&bytes1[i..right_bound], Some(Vectorization::Intel128));
        i += 16;
      }
      assert_eq!(count_utf8_characters(bytes1, Some(Vectorization::Intel128)), count1);
    }

    {
      let mut count2 = 0;
      let mut i = 0;

      while i < bytes2.len() {
        let right_bound = (i + 16).min(bytes2.len());
        count2 += count_utf8_characters(&bytes2[i..right_bound], Some(Vectorization::Intel128));
        i += 16;
      }
      assert_eq!(count_utf8_characters(bytes2, Some(Vectorization::Intel128)), count2);
    }
  }

  #[bench]
  pub fn count_characters_vector128_bench(b: &mut Bencher) {
    let mut count = 0;
    b.iter(|| {
      count += count_utf8_characters(BENCHMARK_INPUT.as_bytes(), Some(Vectorization::Intel128));
    });
  }

  #[bench]
  pub fn count_characters_vector256_bench(b: &mut Bencher) {
    let mut count = 0;
    b.iter(|| {
      count += count_utf8_characters(BENCHMARK_INPUT.as_bytes(), Some(Vectorization::Intel256));
    });
  }

  #[bench]
  pub fn count_characters_vector128_portable_bench(b: &mut Bencher) {
    let mut count = 0;
    b.iter(|| {
      count += count_utf8_characters(BENCHMARK_INPUT.as_bytes(), Some(Vectorization::Portable128));
    });
  }

  #[bench]
  pub fn count_characters_bytes_bench(b: &mut Bencher) {
    let mut count = 0;
    b.iter(|| {
      count += count_utf8_characters_scalar(BENCHMARK_INPUT.as_bytes());
    });
  }

  #[bench]
  pub fn count_characters_chars_bench(b: &mut Bencher) {
    let mut count = 0;
    b.iter(|| {
      count += BENCHMARK_INPUT.chars().count();
    });
  }

  #[bench]
  #[cfg(not(miri))]
  pub fn count_characters_simdutf(b: &mut Bencher) {
    let mut count = 0;
    b.iter(|| {
      count += unsafe { simdutf::count_utf32_from_utf8(BENCHMARK_INPUT.as_bytes()) };
    });
  }
}
