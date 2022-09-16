use std::{
    marker::PhantomPinned,
    ops::{Deref, DerefMut},
    pin::Pin,
    ptr::NonNull,
};

fn main() {}

struct Unmovable {
    data: String,
    slice: NonNull<String>,
    _pin: PhantomPinned,
}

impl Default for Unmovable {
    fn default() -> Self {
        Self {
            data: Default::default(),
            slice: NonNull::dangling(),
            _pin: PhantomPinned,
        }
    }
}

impl Unmovable {
    fn new(data: String) -> Pin<Boz<Self>> {
        let res = Unmovable {
            data,
            slice: NonNull::dangling(),
            _pin: PhantomPinned,
        };
        let mut bozed = unsafe { Pin::new_unchecked(Boz(res)) };
        let slice = NonNull::from(&bozed.data);
        unsafe {
            let mut_ref = Pin::as_mut(&mut bozed);
            Pin::get_unchecked_mut(mut_ref).slice = slice;
        }
        bozed
    }
}

struct Boz<T>(T);
impl<T> Deref for Boz<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: Default> DerefMut for Boz<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let _s = std::mem::take(&mut self.0);
        &mut self.0
    }
}
