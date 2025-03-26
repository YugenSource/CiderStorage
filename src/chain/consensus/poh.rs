use poh_yugen::{PoHConfig, PoHUsage, PoHEntry, TickEntryType, InitialSeed};
use sha2::Sha384;
use sha2::Digest;
use securerand_rs::rngs::FuschineCSPRNG;

pub struct Consensus;

impl Consensus {
    pub fn new(init_data: Vec<u8>) {
        // Initialize the PoH with a configuration
        let config = PoHConfig::new(Sha384::new(), 48, 1000, 1000, false, true, TickEntryType::Empty);
        let seed = FuschineCSPRNG::get_64_bytes_from_os();

        
        // Generate a random seed for the PoH
        let poh = PoHUsage::new(config, seed, init_data, extensions)
        // Example usage of the PoH instance
        println!("PoH instance created: {:?}", poh_instance);
    }
}