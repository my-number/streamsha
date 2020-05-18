use crate::hash_state;
use crate::hash_state::HashState;
pub trait StreamHasher {
    type Output;
    const BLOCK_SIZE: usize;
    /// write to pending block, process block, create new block. Never finish with pending block is filled. End with new empty block if block filled.
    fn update(&mut self, buf: &[u8]) -> usize;
    /// pad and process the last pending block then output the final hash. ハッシュ生成完了した後の動作はTBDです
    fn finish(&mut self) -> Self::Output;
    fn get_block_count(&self) -> usize;
}
pub trait Resumable: Sized {
    fn pause(&self) -> HashState;
    fn resume(hash_state: HashState) -> Result<Self, hash_state::Error>;
}
