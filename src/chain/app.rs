use super::DataPieceID;

pub struct UniversalStorageChain {
    id: u64,
    blocks: Vec<Block>,
    pieces: HashMap<DataPieceID>,
}