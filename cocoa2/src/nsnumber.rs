use objc::runtime::Class;
use objc::{class, msg_send, sel, sel_impl};
use crate::base::id;

pub struct NSNumber(id);

impl NSNumber {
    pub fn class() -> &'static Class {
        class!(NSNumber)
    }

    pub fn numberWithInteger(value: i32) -> NSNumber {
        let class = Self::class();
        unsafe {
            let obj = msg_send![class, numberWithInteger: value];
            NSNumber(obj)
        }
    }

    pub fn intValue(&self) -> isize {
        unsafe {
            msg_send![self.0, intValue]
        }
    }
}
