#![feature(test)]
#![feature(portable_simd)]
#![warn(
  // Pedantic.
  clippy::needless_continue,
  clippy::needless_for_each,
  clippy::needless_pass_by_value,
  clippy::ptr_as_ptr,
  clippy::range_plus_one,
  clippy::range_minus_one,
  clippy::redundant_closure_for_method_calls,
  clippy::redundant_else,
  clippy::similar_names,
  clippy::single_match_else,
  clippy::too_many_lines,
  clippy::uninlined_format_args,
  clippy::unnecessary_join,
  clippy::unnecessary_wraps,
  clippy::unnested_or_patterns,
  clippy::unreadable_literal,
  clippy::unused_self,
  clippy::used_underscore_binding,

  // Nursery.
  clippy::branches_sharing_code,
  clippy::cognitive_complexity,
  clippy::derive_partial_eq_without_eq,
  clippy::empty_line_after_outer_attr,
  clippy::equatable_if_let,
  clippy::fallible_impl_from,
  clippy::manual_clamp,
  clippy::missing_const_for_fn,
  clippy::needless_collect,
  clippy::nonstandard_macro_braces,
  clippy::option_if_let_else,
  clippy::or_fun_call,
  clippy::redundant_pub_crate,
  clippy::string_lit_as_bytes,
  clippy::trait_duplication_in_bounds,
  clippy::trivial_regex,
  clippy::type_repetition_in_bounds,
  clippy::unused_peekable,
  clippy::use_self,
  clippy::useless_let_if_seq
)]
#![deny(clippy::semicolon_if_nothing_returned)]

pub mod types;
pub mod utf16_parser;
pub mod utf32_parser;
pub mod utf8;

#[cfg(test)]
mod tests;
