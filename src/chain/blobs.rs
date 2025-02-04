use crate::constants::BYTES_IN_A_CHUNK;
use crate::constants::{MINIBLOB_SIZE,MINIBLOB_TRIPLE_SIZE};
use crate::chain::DataPieceID;

/// # Blob
/// 
/// A Single Blob Storage Including BLAKE3 of Blob. Blobstrorage of 256kb.
pub struct Blob {
    hash: DataPieceID,
    blob: [u8;BYTES_IN_A_CHUNK],
    nonce: u64,
}

/// # MiniBlob
/// 
/// A single Miniblob Storage Including BLAKE3 of Blob. Blobstorage of 4096bytes.
pub struct MiniBlob {
    hash: DataPieceID,
    miniblob: [u8;MINIBLOB_SIZE],
    nonce: u64,
}

pub struct TripleMiniBlob {
    hash: DataPieceID,
    tripleminiblob: [u8;MINIBLOB_TRIPLE_SIZE]
    nonce: u64,
}