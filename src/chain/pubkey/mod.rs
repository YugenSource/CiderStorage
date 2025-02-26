pub struct PubKey {
    pk: String,
}

impl PubKey {
    pub fn new(pk: String) -> Self {
        PubKey {
            pk,
        }
    }
}