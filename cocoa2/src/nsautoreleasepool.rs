use objc::runtime::Class;
use objc::{class, msg_send, sel, sel_impl};
use crate::base::id;

#[allow(unused)]
pub struct NSAutoreleasePool(id);

impl NSAutoreleasePool {
    pub fn class() -> &'static Class {
        class!(NSAutoreleasePool)
    }

    pub fn new() -> Self {
        unsafe {
            let o = msg_send![Self::class(), new];
            NSAutoreleasePool(o)
        }
    }
}