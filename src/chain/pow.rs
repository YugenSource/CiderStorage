use blake2b_pow::{self, mine};

pub struct UniversalProofOfWork;

impl UniversalProofOfWork {
    pub fn mine<T: AsRef<str>>(hex_str: T, ) -> {
        mine(hex_str.as_ref().as_bytes(), )
    }
}