pub mod myvec;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let vec_ex = Vec::<u8>::new();
        let result = myvec::MyVec::<u8>::default();
    }
}
