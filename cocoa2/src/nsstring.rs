use std::fmt::Debug;
use crate::base::Alloc;
use crate::prelude::*;

const UTF8_ENCODING: usize = 4;

#[derive(Bridge, Display)]
pub struct NSString(pub(crate) id);

impl Debug for NSString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("NSString")
            .field(&self.deref())
            .finish()
    }
}

impl NSString {
    pub fn len(&self) -> usize {
        unsafe {
            msg_send![self.0, lengthOfBytesUsingEncoding: UTF8_ENCODING]
        }
    }

    pub fn UTF8String(&self) -> *const libc::c_char {
        unsafe {
            msg_send![self.0, UTF8String]
        }
    }

    pub fn stringByAppendingString(&self, other: &NSString) -> Self {
        unsafe {
            let obj = msg_send![self.0, stringByAppendingString: other.0];
            NSString(obj)
        }
    }
    pub fn to_str(&self) -> &str {
        self.deref()
    }
}

impl From<&str> for NSString {
    fn from(s: &str) -> Self {
        unsafe {
            let o = Self::class().alloc();
            let o = msg_send![o,
                initWithBytes: s.as_ptr()
                length: s.len()
                encoding:UTF8_ENCODING as id];
            Self(o)
        }
    }
}

impl Deref for NSString {
    type Target = str;

    fn deref(&self) -> &str {
        let len = self.len();
        let ptr = self.UTF8String() as *const u8;
        unsafe {
            let raw_slice = std::slice::from_raw_parts(ptr, len);
            std::str::from_utf8_unchecked(raw_slice)
        }
    }
}