// Copyright 2024 Adam Gutglick

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

// 	http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use core::str;
use std::{borrow::Cow, ops::Deref};

use inline_array::InlineArray;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InlineStr {
    inner: InlineArray,
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
        let as_str: &str = &*self;
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

impl Deref for InlineStr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        // Safety:
        // InlineStr can only be created from valid UTF8 byte sequences
        unsafe { str::from_utf8_unchecked(&self.inner) }
    }
}

impl PartialEq<String> for InlineStr {
    fn eq(&self, other: &String) -> bool {
        (**self).eq(other)
    }
}

impl PartialEq<InlineStr> for String {
    fn eq(&self, other: &InlineStr) -> bool {
        other.eq(self)
    }
}

impl<'a> PartialEq<&'a str> for InlineStr {
    fn eq(&self, other: &&'a str) -> bool {
        (&&**self).eq(other)
    }
}

impl PartialEq<InlineStr> for &str {
    fn eq(&self, other: &InlineStr) -> bool {
        other.eq(self)
    }
}

impl PartialEq<Cow<'_, str>> for InlineStr {
    fn eq(&self, other: &Cow<'_, str>) -> bool {
        (**self).eq(other)
    }
}

impl PartialEq<InlineStr> for Cow<'_, str> {
    fn eq(&self, other: &InlineStr) -> bool {
        other.eq(self)
    }
}

#[cfg(test)]
mod tests {
    use std::hash::{BuildHasher, RandomState};

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
}
