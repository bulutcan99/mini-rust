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

    pub fn grow(&mut self) {
        if self.capacity == 0 {
            self.capacity = 1;
        }

        self.capacity *= 2;
        self.alloc_impl();
    }

    pub fn get_ptr(&self) -> NonNull<T> {
        self.pointer
    }

    fn alloc_impl(&mut self) {
        let layout = Layout::array::<T>(self.capacity as usize).unwrap();

        let raw_ptr: *mut u8 = unsafe {
            if self.get_capacity() == 0 {
                alloc::alloc(layout)
            } else {
                let old_layout = Layout::array::<T>(self.length as usize).unwrap();
                alloc::realloc(self.pointer.as_ptr() as *mut u8, old_layout, layout.size())
            }
        };

        self.pointer = NonNull::new(raw_ptr as *mut T).expect("Failed to allocate memory");
    }
}
