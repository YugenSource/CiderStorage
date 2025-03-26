use crate::constants::BYTES_IN_A_CHUNK;
use crate::constants::{MINIBLOB_SIZE,MINIBLOB_TRIPLE_SIZE};
use crate::chain::DataPieceID;

/// # Blob
/// Storage of 256kb known as a blob.
pub struct Blob {
    hash: DataPieceID,
    storage: [u8;BYTES_IN_A_CHUNK],
    nonce: u64,
}









/*

/// # Blob
/// 
/// A Single Blob Storage Including BLAKE3 of Blob. Blobstrorage of 256kb.
pub struct Blob {
    hash: DataPieceID, // Hash Blob
    bs58_hash: String, // Hash Blob in Base58

    blob: [u8;BYTES_IN_A_CHUNK], // Blob of 256kb
    
    nonce: u64, // Nonce of Blob
}

pub struct BlobBs64 {
    hash: DataPieceID,
    bs64_hash: String,

    blob_bs64: String,
    blob: [u8;BYTES_IN_A_CHUNK],

    nonce: u64,
}

pub struct BlobStorage {
    hash: DataPieceID,
    bs58_hash: String,
    
    blob_bs58: String,
    
    nonce: u64,
    nonce_bs58: u64,

}

impl Blob {
    pub fn new() -> Self {
        Blob {
            hash: DataPieceID::new(),
            bs58_hash: String::new(),
            blob: [0;BYTES_IN_A_CHUNK],
            nonce: 0,
        }
    }
    pub fn blob_from_data(data: [u8;BYTES_IN_A_CHUNK]) -> Self {
        Blob {
            hash: DataPieceID::new(),
            bs58_hash: String::new(),
            blob: data,
            nonce: 0,
        }
    }
    pub fn get_hash(&self) -> DataPieceID {
        self.hash
    }
    pub fn get_bs58_hash(&self) -> String {
        self.bs58_hash.clone()
    }
    pub fn get_blob(&self) -> [u8;BYTES_IN_A_CHUNK] {
        self.blob.clone()
    }
    pub fn get_nonce(&self) -> u64 {
        self.nonce
    }
}

/// # MiniBlob
/// 
/// A single Miniblob Storage Including BLAKE3 of Blob. Blobstorage of 4096bytes.
pub struct TinyBlob {
    hash: DataPieceID,
    miniblob: [u8;MINIBLOB_SIZE],
    nonce: u64,
}

pub struct MiniBlob {
    hash: DataPieceID,
    tripleminiblob: [u8;MINIBLOB_TRIPLE_SIZE],
    nonce: u64,
}

*/