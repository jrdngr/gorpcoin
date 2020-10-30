pub trait BlockData {
    fn into_bytes(&self) -> Vec<u8>;
}

impl BlockData for String {
    fn into_bytes(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl BlockData for &[u8] {
    fn into_bytes(&self) -> Vec<u8> {
        self.to_vec()
    }
}

impl BlockData for Vec<u8> {
    fn into_bytes(&self) -> Vec<u8> {
        self.clone()
    }
}
