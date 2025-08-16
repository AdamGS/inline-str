// Copyright 2025 Adam Gutglick
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A string type that stores strings inline when small.
//!
//! `InlineStr` is a string type built on top of [`inline-array`] that can store small strings
//! directly inline to avoid heap allocation, falling back to heap allocation for larger strings.
//!
//! This crate doesn't do any of the heavy lifting, if you want to better understand how it works
//! its recommended to read through inline-array's docs and source code.
//!
//! # Examples
//!
//! ```
//! use inline_str::InlineStr;
//!
//! let s = InlineStr::from("hello");
//! assert_eq!(s, "hello");
//! ```
//!
//! # Features
//!
//! - **serde**: Enable serialization/deserialization support with serde
//!
//! [`inline-array`]: https://crates.io/crates/inline-array

#![deny(clippy::doc_markdown)]
#![deny(missing_docs)]

use core::str;
use std::{
    borrow::{Borrow, Cow},
    cmp::Ordering,
    ffi::OsStr,
    ops::Deref,
    path::Path,
};

#[cfg(feature = "serde")]
mod serde;

use inline_array::InlineArray;

/// Immutable stack-inlinable string type that can be cheaply cloned and shared.
#[derive(PartialEq, Eq, Clone)]
pub struct InlineStr {
    inner: InlineArray,
}

impl InlineStr {
    /// Extracts a string slice containing the entire `InlineStr`.
    pub fn as_str(&self) -> &str {
        // Safety:
        // InlineStr can only be created from valid UTF8 byte sequences
        unsafe { str::from_utf8_unchecked(&self.inner) }
    }

    /// Returns the length of the `InlineStr` in **bytes**.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns `true` if this `InlineStr` has a length of 0 (in bytes), otherwise `false`.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl std::fmt::Display for InlineStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&**self, f)
    }
}

impl std::fmt::Debug for InlineStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&**self, f)
    }
}

impl std::hash::Hash for InlineStr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let as_str = &**self;
        as_str.hash(state);
    }
}

impl From<String> for InlineStr {
    fn from(value: String) -> Self {
        Self {
            inner: InlineArray::from(value.as_bytes()),
        }
    }
}

impl From<&String> for InlineStr {
    fn from(value: &String) -> Self {
        Self {
            inner: InlineArray::from(value.as_bytes()),
        }
    }
}

impl From<&str> for InlineStr {
    fn from(value: &str) -> Self {
        Self {
            inner: InlineArray::from(value.as_bytes()),
        }
    }
}

impl PartialOrd for InlineStr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for InlineStr {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl PartialEq<String> for InlineStr {
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<InlineStr> for String {
    fn eq(&self, other: &InlineStr) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<&'_ str> for InlineStr {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialEq<InlineStr> for &str {
    fn eq(&self, other: &InlineStr) -> bool {
        *self == other.as_str()
    }
}

impl PartialEq<&InlineStr> for &str {
    fn eq(&self, other: &&InlineStr) -> bool {
        self == *other
    }
}
impl PartialEq<Cow<'_, str>> for InlineStr {
    fn eq(&self, other: &Cow<'_, str>) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<InlineStr> for Cow<'_, str> {
    fn eq(&self, other: &InlineStr) -> bool {
        self.as_ref() == other.as_str()
    }
}

impl PartialEq<InlineStr> for &InlineStr {
    fn eq(&self, other: &InlineStr) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Deref for InlineStr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl AsRef<str> for InlineStr {
    fn as_ref(&self) -> &str {
        self
    }
}

impl AsRef<Path> for InlineStr {
    fn as_ref(&self) -> &Path {
        self.as_str().as_ref()
    }
}

impl AsRef<[u8]> for InlineStr {
    fn as_ref(&self) -> &[u8] {
        self.inner.as_ref()
    }
}

impl AsRef<OsStr> for InlineStr {
    fn as_ref(&self) -> &OsStr {
        self.as_str().as_ref()
    }
}

impl Borrow<str> for InlineStr {
    fn borrow(&self) -> &str {
        self.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        hash::{BuildHasher, RandomState},
    };

    use super::*;

    #[test]
    fn test_basic_eq() {
        let words = "the quick brown fox";
        let inline_words = InlineStr::from(words);

        assert_eq!(words, &*inline_words);
        assert_eq!(words, inline_words);
        assert_eq!(inline_words, words);
    }

    #[test]
    fn test_basic_hash() {
        let hasher = RandomState::new();

        let words = "the quick brown fox";
        let inline_words = InlineStr::from(words);

        let words_hash = hasher.hash_one(words);
        let words_hash_2 = hasher.hash_one(words);
        let inline_hash = hasher.hash_one(inline_words);

        assert_eq!(words_hash, words_hash_2);
        assert_eq!(words_hash, inline_hash);
    }

    #[test]
    fn test_borrow() {
        let map = [(InlineStr::from("x"), 5)]
            .into_iter()
            .collect::<HashMap<InlineStr, i32>>();

        let v = map.get("x");
        assert_eq!(v, Some(&5));
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde() {
        let s = "hello world";
        let inline_s = InlineStr::from("hello world");
        assert_eq!(s, inline_s);
        let serialized_s = serde_json::to_value(s).unwrap();
        let serialized_inline = serde_json::to_value(inline_s.as_str()).unwrap();
        assert_eq!(serialized_s, serialized_inline);
        let deserialized: InlineStr = serde_json::from_value(serialized_s).unwrap();
        assert_eq!(deserialized, "hello world");
    }
}
