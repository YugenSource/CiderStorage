pub const BLAKE2B_DIGEST_SIZE_IN_BYTES: usize = 48usize;
pub const BLAKE2B_DIGEST_SIZE_FILENAME: usize = 8usize;

/// 256 kB (262,144 bytes) | assumes 1kB is equal to 1024 bytes | 262144
pub const BYTES_IN_A_CHUNK: usize = 262_144;

pub const FILENAME_SIZE: usize = 8;

//
pub const DIFFICULTY_LOWEST: &str = "0000";
pub const DIFFICULTY_MEDIUM: &str = "000000";
pub const DIFFICULTY_HIGHEST: &str = "00000000";


/// # Chain Constants
pub const MINIBLOB_SIZE: usize = 4_096;
pub const MINIBLOB_DOUBLE_SIZE: usize = 8_192;
pub const MINIBLOB_TRIPLE_SIZE: usize = 16_384;