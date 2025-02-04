use super::blobs::TripleMiniBlob;
use super::blobs::MiniBlob;
use super::blobs::Blob;

pub struct Block {
    prev_hash: String,
    pieces: Vec<DataPieceID>,
    
    blobs: Vec<BlobStorage>,
    miniblobs: Vec<MiniBlob>,
    tripleminiblobs: Vec<TripleMiniBlob
    
    
    /// PoW
    nonce: u64,
}