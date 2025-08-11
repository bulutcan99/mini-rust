use std::{
    alloc::{self, GlobalAlloc, Layout},
    ptr::NonNull,
};

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

impl<T> MyVec<T> {
    pub fn get_capacity(&self) -> u8 {
        self.capacity
    }

    fn set_capacity(&mut self, new_cap: u8) -> u8 {
        self.capacity = new_cap;
        self.capacity
    }

    pub fn get_pointer(&self) -> NonNull<T> {
        self.pointer
    }

    fn set_pointer(&mut self, new_ptr: NonNull<T>) {
        self.pointer = new_ptr;
    }

    pub fn grow(&mut self) {
        let old_cap = self.get_capacity();
        let new_cap = if old_cap == 0 { 1 } else { old_cap * 2 };

        let new_layout = Layout::array::<T>(new_cap as usize).unwrap();

        let raw_ptr = if old_cap == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(old_cap as usize).unwrap();
            unsafe {
                alloc::realloc(
                    self.get_pointer().as_ptr() as *mut u8,
                    old_layout,
                    new_layout.size(),
                )
            }
        };

        let new_ptr = NonNull::<T>::new(raw_ptr as *mut T).expect("Failed to allocate memory");

        self.set_pointer(new_ptr);
        self.set_capacity(new_cap);
    }

    pub fn get_ptr(&self) -> NonNull<T> {
        self.pointer
    }
}
