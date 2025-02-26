use blake2b_pow;

pub struct UniversalProofOfWork;

impl UniversalProofOfWork {
    pub fn mine_blob<T: AsRef<str>>(hex_str: T) -> u64 {
        mine(hex_str.as_ref().as_bytes(), 0xffffffc000000000)
    }
}