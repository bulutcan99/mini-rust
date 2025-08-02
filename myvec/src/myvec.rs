use std::ptr::NonNull;

pub struct MyVec<T> {
    pub pointer: NonNull<T>,
    length: u8,
    capacity: u8,
}

impl<T> Default for MyVec<T> {
    fn default() -> Self {
        Self {
            pointer: NonNull::dangling(),
            length: 0,
            capacity: 0,
        }
    }
}
