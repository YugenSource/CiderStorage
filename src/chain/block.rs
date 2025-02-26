use super::blobs::MiniBlob;
use super::blobs::TinyBlob;
use super::blobs::Blob;

pub struct Block {
    prev_hash: String,
    
    pieces: Vec<DataPieceID>,
    
    blobs: Vec<BlobStorage>, // Blobs of 256kb
    miniblobs: Vec<MiniBlob>, // Blobs of 
    tripleminiblobs: Vec<TripleMiniBlob>,
    
    
    /// PoW
    nonce: u64,
}

pub struct BlockConfig {
    num_of_blobs: u8,
}