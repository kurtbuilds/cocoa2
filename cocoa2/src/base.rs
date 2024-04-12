use std::ffi::c_void;

use objc::{msg_send, runtime, sel, sel_impl};
use objc::runtime::{Class, Object, Sel};

pub type SEL = runtime::Sel;

pub const NULL: *const c_void = 0 as *const c_void;

pub use cocoa2_base::id;

#[cfg(target_pointer_width = "64")]
pub type CGFloat = libc::c_double;
#[cfg(not(target_pointer_width = "64"))]
pub type CGFloat = libc::c_float;

#[cfg(target_pointer_width = "32")]
pub type NSInteger = libc::c_int;
#[cfg(target_pointer_width = "32")]
pub type NSUInteger = libc::c_uint;

#[cfg(target_pointer_width = "64")]
pub type NSInteger = libc::c_long;
#[cfg(target_pointer_width = "64")]
pub type NSUInteger = libc::c_ulong;

#[allow(non_upper_case_globals)]
pub const nil: id = 0 as id;

#[inline]
pub fn selector(name: &str) -> SEL {
    Sel::register(name)
}

pub trait OptionSelectorExt {
    fn or_null(self) -> SEL;
}

impl OptionSelectorExt for Option<SEL> {
    fn or_null(self) -> SEL {
        self.unwrap_or(unsafe { Sel::from_ptr(NULL) })
    }
}

pub trait AutoRelease {
    unsafe fn autorelease(self) -> Self;
}

impl AutoRelease for *mut Object {
    unsafe fn autorelease(self) -> Self {
        unsafe {
            msg_send![self, autorelease]
        }
    }
}

pub trait Init {
    unsafe fn init(self) -> id;
}

impl Init for id {
    unsafe fn init(self) -> id {
        unsafe {
            msg_send![self, init]
        }
    }
}

pub trait Alloc {
    unsafe fn alloc(self) -> id;
    unsafe fn new(self) -> id;
}

impl Alloc for &'static Class {
    unsafe fn alloc(self) -> id {
        unsafe {
            msg_send![self, alloc]
        }
    }

    unsafe fn new(self) -> id {
        unsafe {
            msg_send![self, new]
        }
    }
}