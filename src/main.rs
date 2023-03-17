#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_must_use)]

mod std {
    use std::{fmt::Display, ops::Shl};

    pub struct cout;
    impl<T: Display> Shl<T> for cout {
        type Output = cout;

        fn shl(self, rhs: T) -> cout {
            print!("{}", rhs);
            self
        }
    }

    pub const endl: &str = "\n";
}

fn main() {
    std::cout << "Hello " << 123 << std::endl;
}
