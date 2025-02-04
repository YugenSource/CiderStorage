use super::DataPieceID;
use std::collections::HashMap;

use super::block::Block;

pub struct UniversalStorageChain {
    pubkey_address: String,

    // Chains 0-F (16)
    blocks_0: Vec<Block>,
    blocks_1: Vec<Block>,
    blocks_2: Vec<Block>,
    blocks_3: Vec<Block>,
    blocks_4: Vec<Block>,
    blocks_5: Vec<Block>,
    blocks_6: Vec<Block>,
    blocks_7: Vec<Block>,
    blocks_8: Vec<Block>,
    blocks_9: Vec<Block>,
    blocks_A: Vec<Block>,
    blocks_B: Vec<Block>,
    blocks_C: Vec<Block>,
    blocks_D: Vec<Block>,
    blocks_E: Vec<Block>,
    blocks_F: Vec<Block>,
    
    // Nodes
    pieces: HashMap<DataPieceID,DataPieceID>,
    pieces_id: HashMap<DataPieceID,(u8,u64)>

    // Mempool
    mempool:
}