use super::blobs::Blob;
use crate::chain::DataPieceID;

pub struct Block {
    prev_hash: String,
    
    pieces: Vec<DataPieceID>,
    
    blobs: Vec<Blob>, // Blobs of 256kb
    
    /// PoW
    nonce: u64,
}