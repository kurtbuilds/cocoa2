use objc::runtime::Class;
use objc::{class, msg_send, sel, sel_impl};
use crate::base::id;
use crate::nsstring::NSString;

pub struct NSProcessInfo(id);

impl NSProcessInfo {
    pub fn class() -> &'static Class {
        class!(NSProcessInfo)
    }

    pub fn processInfo() -> Self {
        let class = Self::class();
        unsafe {
            let obj = msg_send![class, processInfo];
            Self(obj)
        }
    }

    pub fn processName(&self) -> NSString {
        unsafe {
            let o = msg_send![self.0, processName];
            NSString(o)
        }
    }
}
