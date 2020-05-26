use crate::hash_state;
use crate::hash_state::HashState;
pub trait StreamHasher {
    type Output;
    const BLOCK_SIZE: usize;
    /// write to pending block, process block, create new block. Never finish with pending block is filled. End with new empty block if block filled.
    fn update(&mut self, buf: &[u8]) -> usize;
    /// Pad and process the last pending block then output the final hash.
    /// NOTE: After finished, the struct will be moved out, making it unreusable.
    fn finish(self) -> Self::Output;
}
pub trait Resumable: Sized {
    /// Returns the current hash state.
    /// note: It returns raw data of block if the block is incomplete.
    fn pause(&self) -> HashState;
    /// Recreate new instance from hash state.
    fn resume(hash_state: HashState) -> Result<Self, hash_state::Error>;
}
