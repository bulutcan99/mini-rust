pub mod myvec;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = myvec::MyVec::<u8>::default();
    }
}
