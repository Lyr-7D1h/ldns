use std::cell::RefCell;

thread_local! {
    // Thread local variable
    pub static SEED_U16: RefCell<u16> = RefCell::new(500);
}

/// An implementation of xorshift (https://en.wikipedia.org/wiki/Xorshift) for generating random numbers with a predefined seed
pub fn xorshift_u16() -> u16 {
    SEED_U16.with(|x| {
        let mut x = x.borrow_mut();
        *x ^= *x << 7;
        *x ^= *x >> 9;
        *x ^= *x << 8;

        x.clone()
    })
}

#[cfg(test)]
mod tests {
    use crate::xorshift_u16;

    #[test]
    fn xorshift_u16_works() {
        let a = xorshift_u16();
        let b = xorshift_u16();

        assert_ne!(a, b);

        let a1 = xorshift_u16();
        let b1 = xorshift_u16();

        assert_ne!(a, a1);
        assert_ne!(b, b1);
        assert_ne!(a1, b1);

        println!("{a}, {b}");
    }
}
