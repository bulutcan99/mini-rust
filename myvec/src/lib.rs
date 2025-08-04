pub mod myvec;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let vec_ex = Vec::<u8>::new();
        let mut result = myvec::MyVec::<u8>::default();
        println!("Cap: {:?}", result.get_capacity());
        println!("PTR: {:?}", result.get_ptr());
        result.grow();
        println!("New Cap: {:?}", result.get_capacity());
        println!("New PTR: {:?}", result.get_ptr());
    }
}
