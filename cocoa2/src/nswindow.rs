use bitflags::bitflags;

use cocoa2_base::Any;

use crate::base::{Alloc, AutoRelease, nil, NSUInteger};
use crate::describe::Describe;
use crate::geometry::NSRect;
use crate::NSSize;
use crate::nsstring::NSString;
use crate::prelude::*;

#[derive(Bridge, Debug)]
pub struct NSWindow(pub(crate) id);

impl std::fmt::Debug for Describe<'_, NSWindow> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let this = self.0;
        f.debug_struct("NSWindow")
            .field("id", &this.0)
            .field("key_window", &this.isKeyWindow())
            .field("main_window", &this.isMainWindow())
            .field("frame", &this.frame())
            .field("title", &this.title())
            .finish()
    }
}

impl NSWindow {
    pub fn describe(&self) -> Describe<'_, Self> {
        Describe(self)
    }

    pub fn newWith(rect: NSRect, style: StyleMask, store_type: NSBackingStoreType, defer: bool) -> Self {
        unsafe {
            let o = Self::class().alloc();
            let o: id = msg_send![o, initWithContentRect:rect styleMask:style backing:store_type defer:defer];
            o.autorelease();
            Self(o)
        }
    }

    pub fn center(&self) {
        unsafe {
            msg_send![self.0, center]
        }
    }

    pub fn setTitle(&self, title: &NSString) {
        unsafe {
            msg_send![self.0, setTitle: title.0]
        }
    }
    pub fn makeKeyAndOrderFront(&self) {
        unsafe {
            msg_send![self.0, makeKeyAndOrderFront:nil]
        }
    }

    pub fn isKeyWindow(&self) -> bool {
        unsafe {
            msg_send![self.0, isKeyWindow]
        }
    }

    pub fn canBecomeKeyWindow(&self) -> bool {
        unsafe {
            msg_send![self.0, canBecomeKeyWindow]
        }
    }

    pub fn makeKeyWindow(&self) {
        unsafe {
            msg_send![self.0, makeKeyWindow]
        }
    }

    pub fn canBecomeMainWindow(&self) -> bool {
        unsafe {
            msg_send![self.0, canBecomeMainWindow]
        }
    }

    pub fn isMainWindow(&self) -> bool {
        unsafe {
            msg_send![self.0, isMainWindow]
        }
    }

    pub fn makeMainWindow(&self) {
        unsafe {
            msg_send![self.0, makeMainWindow]
        }
    }

    pub fn order_out(&self, sender: &dyn Any) {
        unsafe {
            msg_send![self.0, orderOut: sender.to_id()]
        }
    }

    pub fn frame(&self) -> NSRect {
        unsafe {
            let frame: NSRect = msg_send![self.0, frame];
            frame
        }
    }

    pub fn title(&self) -> NSString {
        unsafe {
            NSString(msg_send![self.0, title])
        }
    }

    pub fn setMinSize(&self, size: NSSize) {
        unsafe {
            msg_send![self.0, setMinSize: size]
        }
    }

    pub fn setMaxSize(&self, size: NSSize) {
        unsafe {
            msg_send![self.0, setMaxSize: size]
        }
    }
}

bitflags! {
    pub struct StyleMask: NSUInteger {
        const Borderless      = 0;
        const Titled          = 1 << 0;
        const Closable        = 1 << 1;
        const Miniaturizable  = 1 << 2;
        const Resizable       = 1 << 3;

        const TexturedBackground  = 1 << 8;

        const UnifiedTitleAndToolbar  = 1 << 12;

        const FullScreen      = 1 << 14;

        const NSFullSizeContentView = 1 << 15;
    }
}

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSBackingStoreType {
    Retained = 0,
    Nonretained = 1,
    Buffered = 2,
}