use crate::constants::BYTES_IN_A_CHUNK;
use crate::constants::{MINIBLOB_SIZE,MINIBLOB_TRIPLE_SIZE};

use base58::{FromBase58,ToBase58};

/// Blobs
pub mod blobs;

pub mod app;

pub struct UniversalStorageChain {
    pub chains: Vec<Block>,
    
    pub pieces: HashMap<String,DataPieceID>,
}

impl UniversalStorageChain {
    pub fn genesis() {

    }
}

pub type DataPieceID = String;

pub struct BlobStorage {
    hash: DataPieceID,
    blob: [u8;BYTES_IN_A_CHUNK],
    nonce: u64,
}

impl BlobStorage {
    pub fn verify(&self) -> bool {
        if blake3::hash(&self.blob).to_string() == self.hash {
            return true
        }
        else {
            return false
        }
    }
    pub fn blob_to_bs58(&self) -> String {
        self.blob.to_base58()
    }
}

pub struct Block {
    prev_hash: String,
    pieces: Vec<DataPieceID>,
    
    blobs: Vec<BlobStorage>,
    miniblobs: Vec<MiniBlob>,
    miniblobs: Vec<
    
    
    /// PoW
    nonce: u64,
}

impl Block {
    pub fn genesis() {

        Self {
            prev_hash: String::from("Genesis"),
            pieces:
        } 
    }
    

}