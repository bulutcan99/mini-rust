use std::ptr::{null, NonNull};

fn main() {
    println!("Hello, dangling!");
    let new_vec = MyVec::<u8>::new();
    println!("DANGLED: {:?}", new_vec.pointer);
}

struct MyVec<T> {
    pub pointer: NonNull<T>,
    length: u8,
    capacity: u8,
}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        Self {
            pointer: NonNull::dangling(),
            length: 0,
            capacity: 0,
        }
    }
}
