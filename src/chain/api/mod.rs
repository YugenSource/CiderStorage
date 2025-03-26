pub struct OpenImmutableStorageChain;

pub mod traits;

pub struct OISCConfig {
    pub pubkey_address: String,
    pub csprng_256: String,
}

impl OpenImmutableStorageChain {
    pub fn new() -> Self {
        OpenImmutableStorageChain {}
    }
    pub fn genesis() -> Self {
        OpenImmutableStorageChain {}
    }
}

