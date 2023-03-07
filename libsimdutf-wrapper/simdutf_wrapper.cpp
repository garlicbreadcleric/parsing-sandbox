#include <simdutf.h>

extern "C" {
  size_t utf16_length_from_utf8(const char * input, size_t length) {
    return simdutf::utf16_length_from_utf8(input, length);
  }

  size_t convert_valid_utf8_to_utf16le(const char * input, size_t length, char16_t* utf16_buffer) {
    return simdutf::convert_valid_utf8_to_utf16le(input, length, utf16_buffer);
  }

  size_t count_utf8(const char * input, size_t length) {
    return simdutf::count_utf8(input, length);
  }
}