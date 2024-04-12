use crate::prelude::*;

#[derive(Bridge)]
pub struct NSRunningApplication(pub(crate) id);

impl NSRunningApplication {
    pub fn currentApplication() -> Self {
        let class = Self::class();
        unsafe {
            let obj = msg_send![class, currentApplication];
            Self(obj)
        }
    }

    pub fn activateWithOptions(&self, options: NSApplicationActivationOptions) -> bool {
        unsafe {
            msg_send![self.0, activateWithOptions: options]
        }
    }
}

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSApplicationActivationOptions {
    AllWindows = 1 << 0,
    IgnoringOtherApps = 1 << 1,
}