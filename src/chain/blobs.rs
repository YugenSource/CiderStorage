use crate::constants::BYTES_IN_A_CHUNK;
use crate::constants::{MINIBLOB_SIZE,MINIBLOB_DOUBLE_SIZE,MINIBLOB_TRIPLE_SIZE};

/// # Blob
/// 
/// A Single Blob Storage Including BLAKE3 of Blob
pub struct Blob {
    hash: DataPieceID,
    blob: [u8;BYTES_IN_A_CHUNK],
}