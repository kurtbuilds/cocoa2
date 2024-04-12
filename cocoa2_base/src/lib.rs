use objc::runtime::{Class, Object};

#[allow(non_camel_case_types)]
pub type id = *mut Object;

pub trait Bridge {
    /// Give a reference to the objc class object for class methods
    /// Returns same objects as `class!(NSString)` in `objc` crate.
    /// E.g. `Self::class().alloc()`
    fn class() -> &'static Class;
    fn id(&self) -> id;
    unsafe fn from_id(id: id) -> Self;
}

pub trait Any {
    fn to_id(&self) -> id;
}

impl Any for id {
    fn to_id(&self) -> id {
        *self
    }
}

impl<B: Bridge> Any for B {
    fn to_id(&self) -> id {
        self.id()
    }
}