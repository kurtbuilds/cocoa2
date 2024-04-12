use std::fmt::{Debug, Formatter};
use crate::base::{Alloc, AutoRelease, id};
use crate::nsmenuitem::NSMenuItem;
use crate::nsstring::NSString;
use crate::NSUInteger;
use crate::describe::Describe;
use crate::prelude::*;

#[derive(Bridge)]
pub struct NSMenu(pub(crate) id);

impl<'a> Debug for Describe<'a, NSMenu> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let this = self.0;
        f.debug_struct("NSMenu")
            .field("id", &this.0)
            .field("title", &this.title())
            .field("items", &Describe(&this.items()))
            .finish()
    }
}

impl NSMenu {
    pub fn describe(&self) -> Describe<'_, NSMenu> {
        Describe(self)
    }

    pub fn new() -> Self {
        unsafe {
            Self(Self::class().new().autorelease())
        }
    }

    pub fn newWithTitle(title: &NSString) -> Self {
        unsafe {
            let o = Self::class().alloc();
            let o: id = msg_send![o, initWithTitle: title.0];
            Self(o.autorelease())
        }
    }

    pub fn addItem(&self, item: &NSMenuItem) {
        unsafe {
            msg_send![self.0, addItem: item.0]
        }
    }

    pub fn items(&self) -> Vec<NSMenuItem> {
        unsafe {
            let items: id = msg_send![self.0, itemArray];
            let count: NSUInteger = msg_send![items, count];
            let mut result = Vec::with_capacity(count as usize);
            for i in 0..count {
                let item: id = msg_send![items, objectAtIndex: i];
                result.push(NSMenuItem(item));
            }
            result
        }
    }

    pub fn title(&self) -> NSString {
        unsafe {
            let title: id = msg_send![self.0, title];
            NSString(title)
        }
    }
}