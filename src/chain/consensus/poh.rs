use poh_yugen::{PoHConfig, PoHUsage, PoHEntry, TickEntryType, InitialSeed};
use sha2::Sha384;
use sha2::Digest;
use securerand_rs::rngs::FuschineCSPRNG;

pub struct Consensus;

impl Consensus {
    pub fn new(init_data: Vec<u8>) {
        // Initialize the PoH with a configuration
        let config = PoHConfig::new(Sha384::new(), 48, 1000, Some(1000), false, true, TickEntryType::Empty);
        let seed = FuschineCSPRNG::get_64_bytes_from_os();
        let init_seed = InitialSeed(seed);

        
        // Generate a random seed for the PoH
        let mut poh = PoHUsage::new(config, init_seed, Some(init_data), vec![]);
        let x = poh.init();
        // Example usage of the PoH instance
        let state = poh.get_state();
        let entry = hex::encode(&state[0].hash);
        println!("PoH Entry: {}", entry);
        let entry1000 = hex::encode(&state[1000].hash);
        println!("PoH Entry (1000): {}", entry1000);
    }
}

#[test]
fn run() {
    Consensus::new(vec![0; 256]);
}