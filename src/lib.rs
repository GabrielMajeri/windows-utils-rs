//! Utility functions and structures for working with the Windows API.

#![cfg(windows)]
#![deny(warnings, missing_docs)]

use std::{string, slice};

/// Converts a string slice to a null-terminated wide (UTF-16) string.
///
/// Pass the pointer from calling `as_ptr` on the returned vector to the native function.
///
/// # Example
/// ```rust, norun
///
/// ```
pub fn str_to_wide(input: &str) -> Vec<u16> {
	input.encode_utf16()
		.chain(Some(0))
		.collect()
}

/// Converts a null-terminated wide string to a regular Rust UTF-8 string.
pub fn wide_str_nul_to_string(input: *const u16) -> Result<String, string::FromUtf16Error> {
	let mut length = 0;

	while unsafe { *input.offset(length) } != 0 {
		length += 1;
	}

	let input = unsafe { slice::from_raw_parts(input, length as usize) };

	String::from_utf16(input)
}
