use std::fmt::Debug;
use crate::base::{Alloc, AutoRelease, id, SEL};
use crate::describe::Describe;
use crate::nsmenu::NSMenu;
use crate::nsstring::NSString;
use crate::prelude::*;

#[derive(Bridge)]
pub struct NSMenuItem(pub(crate) id);

impl Debug for Describe<'_, NSMenuItem> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let this = self.0;
        f.debug_struct("NSMenuItem")
            .field("id", &this.0)
            .field("title", &this.title())
            .field("submenu", &Describe(&this.submenu()))
            .field("keyEquivalent", &this.keyEquivalent())
            .finish()
    }
}

impl NSMenuItem {
    pub fn describe(&self) -> Describe<'_, NSMenuItem> {
        Describe(self)
    }

    pub fn new() -> Self {
        unsafe {
            Self(Self::class().new().autorelease())
        }
    }

    pub fn newWith(title: &str, action: SEL, keyEquivalent: &str) -> Self {
        let title = NSString::from(title);
        let keyEquivalent = NSString::from(keyEquivalent);
        unsafe {
            let o = Self::class().alloc();
            let o: id = msg_send![o, initWithTitle:title action:action keyEquivalent:keyEquivalent];
            Self(o.autorelease())
        }
    }

    pub fn setSubmenu(&self, submenu: &NSMenu) {
        unsafe {
            msg_send![self.0, setSubmenu: submenu.0]
        }
    }

    pub fn submenu(&self) -> Option<NSMenu> {
        unsafe {
            let submenu: id = msg_send![self.0, submenu];
            if submenu.is_null() {
                None
            } else {
                Some(NSMenu(submenu))
            }
        }
    }

    pub fn title(&self) -> NSString {
        unsafe {
            let title: id = msg_send![self.0, title];
            NSString(title)
        }
    }

    pub fn keyEquivalent(&self) -> NSString {
        unsafe {
            let keyEquivalent: id = msg_send![self.0, keyEquivalent];
            NSString(keyEquivalent)
        }
    }
}