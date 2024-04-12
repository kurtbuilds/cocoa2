use crate::base::CGFloat;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct NSRect(pub NSPoint, pub NSSize);

impl NSRect {
    pub fn new(x: CGFloat, y: CGFloat, width: CGFloat, height: CGFloat) -> Self {
        NSRect(NSPoint(x, y), NSSize(width, height))
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct NSPoint(pub CGFloat, pub CGFloat);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct NSSize(pub CGFloat, pub CGFloat);