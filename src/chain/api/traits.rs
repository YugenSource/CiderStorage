use crate::chain::block::Block;

pub trait StorageChain {
    fn new() -> Self;
    fn genesis() -> Self;
    fn get_chain_number(&self) -> u64;
    fn get_chain(&self) -> Vec<Block>;
}