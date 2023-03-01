use std::arch::x86_64::*;
use std::simd::Simd;

#[cfg(test)]
enum Vectorization {
  V128,
  V256,
}

#[cfg(test)]
fn count_utf8_characters(bytes: &[u8], vectorization: Option<Vectorization>) -> usize {
  let mut i = 0;
  let mut count = 0;
  match vectorization {
    Some(Vectorization::V128) => {
      while i + 15 < bytes.len() {
        let v = unsafe { _mm_loadu_si128((bytes[i..].as_ptr()).cast()) };
        count += count_utf8_characters_v128(v);
        i += 16;
      }
    }
    Some(Vectorization::V256) => {
      while i + 31 < bytes.len() {
        let v = unsafe { _mm256_loadu_si256((bytes[i..].as_ptr()).cast()) };
        count += count_utf8_characters_v256(v);
        i += 32;
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
  if starting_byte < 0b1000_0000 {
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

  use super::*;
  use crate::test_data::*;

  #[test]
  pub fn count_characters_test() {
    let bytes1 = SHORT_ASCII_INPUT.as_bytes();
    let bytes2 = SHORT_UNICODE_INPUT.as_bytes();

    assert_eq!(count_utf8_characters(bytes1, Some(Vectorization::V128)), SHORT_ASCII_INPUT.chars().count());
    assert_eq!(count_utf8_characters(bytes2, Some(Vectorization::V128)), SHORT_UNICODE_INPUT.chars().count());

    assert_eq!(count_utf8_characters(bytes1, Some(Vectorization::V256)), SHORT_ASCII_INPUT.chars().count());
    assert_eq!(count_utf8_characters(bytes2, Some(Vectorization::V256)), SHORT_UNICODE_INPUT.chars().count());

    assert_eq!(count_utf8_characters_scalar(bytes1), SHORT_ASCII_INPUT.chars().count());
    assert_eq!(count_utf8_characters_scalar(bytes2), SHORT_UNICODE_INPUT.chars().count());

    {
      let mut count1 = 0;
      let mut i = 0;

      while i < bytes1.len() {
        let right_bound = (i + 16).min(bytes1.len());
        count1 += count_utf8_characters(&bytes1[i..right_bound], Some(Vectorization::V128));
        i += 16;
      }
      assert_eq!(count_utf8_characters(bytes1, Some(Vectorization::V128)), count1);
    }

    {
      let mut count2 = 0;
      let mut i = 0;

      while i < bytes2.len() {
        let right_bound = (i + 16).min(bytes2.len());
        count2 += count_utf8_characters(&bytes2[i..right_bound], Some(Vectorization::V128));
        i += 16;
      }
      assert_eq!(count_utf8_characters(bytes2, Some(Vectorization::V128)), count2);
    }
  }

  #[bench]
  pub fn count_characters_vector128_bench(b: &mut Bencher) {
    let mut count = 0;
    b.iter(|| {
      count += count_utf8_characters(BENCHMARK_INPUT.as_bytes(), Some(Vectorization::V128));
    });
  }

  #[bench]
  pub fn count_characters_vector256_bench(b: &mut Bencher) {
    let mut count = 0;
    b.iter(|| {
      count += count_utf8_characters(BENCHMARK_INPUT.as_bytes(), Some(Vectorization::V256));
    });
  }

  #[bench]
  pub fn count_characters_scalar_bench(b: &mut Bencher) {
    let mut count = 0;
    b.iter(|| {
      count += count_utf8_characters_scalar(BENCHMARK_INPUT.as_bytes());
    });
  }

  #[bench]
  pub fn count_characters_native_bench(b: &mut Bencher) {
    let mut count = 0;
    b.iter(|| {
      count += BENCHMARK_INPUT.chars().count();
    });
  }
}
