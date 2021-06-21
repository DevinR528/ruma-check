macro_rules! mac_exp {
    ($name:ident, $call:expr) => {
        pub struct $name {
            a: usize,
            b: String,
        }
        impl $name {
            fn method(&self) { $call }
        }
    };
}
use crate::mac_exp;

mac_exp! { Test, let x = self.a + 1; }
