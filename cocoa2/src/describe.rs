use std::fmt::Debug;

/// A wrapper type that implements `Debug` for a reference to a type.
/// Use this for types that call methods that must be called on the main thread.
pub struct Describe<'a, T>(pub &'a T);

impl<'a, T> Debug for Describe<'a, Vec<T>> where Describe<'a, T>: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.0.iter().map(Describe)).finish()
    }
}

impl<'a, T> Debug for Describe<'a, Option<T>> where Describe<'a, T>: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(x) => f.debug_tuple("Some").field(&Describe(x)).finish(),
            None => f.debug_tuple("None").finish(),
        }
    }
}