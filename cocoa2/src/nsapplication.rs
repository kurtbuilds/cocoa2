use objc::{class, msg_send, sel, sel_impl};
use objc::runtime::Class;

use crate::base::id;
use crate::nsmenu::NSMenu;
use crate::nswindow::NSWindow;

pub struct NSApplication(pub(crate) id);

impl NSApplication {
    pub fn class() -> &'static Class {
        class!(NSApplication)
    }

    // apparently needs to be called from main thread
    // https://stackoverflow.com/questions/60163972/is-it-ok-accessing-uiapplication-sharedapplication-from-background-thread
    pub fn sharedApplication() -> Self {
        unsafe {
            let o = msg_send![Self::class(), sharedApplication];
            Self(o)
        }
    }
    pub fn setActivationPolicy(&self, policy: ActivationPolicy) -> bool {
        unsafe {
            msg_send![self.0, setActivationPolicy: policy as i64]
        }
    }

    pub fn setMainMenu(&self, menu: &NSMenu) {
        unsafe {
            msg_send![self.0, setMainMenu: menu.0]
        }
    }

    pub fn run(&self) {
        unsafe {
            msg_send![self.0, run]
        }
    }

    pub fn windows(&self) -> Vec<NSWindow> {
        unsafe {
            let windows: id = msg_send![self.0, windows];
            let count: usize = msg_send![windows, count];
            let mut result = Vec::with_capacity(count);
            for i in 0..count {
                let window: id = msg_send![windows, objectAtIndex: i];
                result.push(NSWindow(window));
            }
            result
        }
    }

    pub fn mainMenu(&self) -> NSMenu {
        unsafe {
            let menu: id = msg_send![self.0, mainMenu];
            NSMenu(menu)
        }
    }
}

#[repr(i64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActivationPolicy {
    Regular = 0,
    Accessory = 1,
    Prohibited = 2,
    ERROR = -1,
}