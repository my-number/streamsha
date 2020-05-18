use crate::consts::*;
pub enum HashState {
    Sha256(Sha256HashState),
}
pub struct Sha256HashState {
    pub h: [u32; 8],
    pub message_len: u64,
    pub block_len: usize,
    pub current_block: [u8; SHA256_BLOCK_SIZE],
}
#[derive(Debug)]
pub enum Error{
    HashTypeNotMatch
}
