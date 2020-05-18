use crate::arith::*;
use crate::consts::*;
use crate::hash_state;
use crate::hash_state::HashState;
pub use crate::traits::*;
#[derive(Clone)]
pub struct Sha256 {
    /// hash value
    h: [Word32; 8],
    /// max length of message (in bytes) defined in fips 180-4
    message_len: u64,
    /// the length of `current_block` in bytes
    block_len: usize,
    /// incomplete block that is waiting to be filled and hashed
    current_block: [u8; SHA256_BLOCK_SIZE],
}

impl Sha256 {
    pub fn new() -> Self {
        Default::default()
    }
    /// compute hash for current block
    fn process_block(&mut self) {
        if self.block_len != SHA256_BLOCK_SIZE {
            panic!("block is not filled");
        }
        let mut w = [Word32(0); 64];
        for t in 0..16 {
            w[t] = self.get_word32_in_block(t)
        }
        for t in 16..64 {
            w[t] = Self::lsigma1(w[t - 2]) + w[t - 7] + Self::lsigma0(w[t - 15]) + w[t - 16];
        }
        let mut a = self.h[0];
        let mut b = self.h[1];
        let mut c = self.h[2];
        let mut d = self.h[3];
        let mut e = self.h[4];
        let mut f = self.h[5];
        let mut g = self.h[6];
        let mut h = self.h[7];

        for t in 0..64 {
            let t1 = h + Self::sigma1(e) + Self::ch(e, f, g) + SHA256_K[t] + w[t];
            let t2 = Self::sigma0(a) + Self::maj(a, b, c);
            h = g;
            g = f;
            f = e;
            e = d + t1;
            d = c;
            c = b;
            b = a;
            a = t1 + t2;
        }
        self.h[0] = a + self.h[0];
        self.h[1] = b + self.h[1];
        self.h[2] = c + self.h[2];
        self.h[3] = d + self.h[3];
        self.h[4] = e + self.h[4];
        self.h[5] = f + self.h[5];
        self.h[6] = g + self.h[6];
        self.h[7] = h + self.h[7];

        self.current_block = [0u8; SHA256_BLOCK_SIZE]; // next block
        self.block_len = 0; // reset block
    }
    fn get_word32_in_block(&self, i: usize) -> Word32 {
        let m: u32 = ((self.current_block[i * 4] as u32) << 24)
            + ((self.current_block[i * 4 + 1] as u32) << 16)
            + ((self.current_block[i * 4 + 2] as u32) << 8)
            + (self.current_block[i * 4 + 3] as u32);
        Word32(m)
    }
}
impl Sha256 {
    fn sigma0(x: Word32) -> Word32 {
        rotr(x, 2) ^ rotr(x, 13) ^ rotr(x, 22)
    }
    fn sigma1(x: Word32) -> Word32 {
        rotr(x, 6) ^ rotr(x, 11) ^ rotr(x, 25)
    }
    fn lsigma0(x: Word32) -> Word32 {
        rotr(x, 7) ^ rotr(x, 18) ^ (x >> 3)
    }
    fn lsigma1(x: Word32) -> Word32 {
        rotr(x, 17) ^ rotr(x, 19) ^ (x >> 10)
    }
    fn ch(x: Word32, y: Word32, z: Word32) -> Word32 {
        (x & y) ^ (!x & z)
    }
    fn maj(x: Word32, y: Word32, z: Word32) -> Word32 {
        (x & y) ^ (x & z) ^ (y & z)
    }
}
impl StreamHasher for Sha256 {
    type Output = [u8; 32];
    const BLOCK_SIZE: usize = SHA256_BLOCK_SIZE;
    fn update(&mut self, buf: &[u8]) -> usize {
        let len = buf.len();
        if len == 0 {
            // if no data or no remaining data, stop
            return 0;
        }
        let writable_len = Self::BLOCK_SIZE - self.block_len;
        let writable_area = &mut self.current_block[self.block_len..];

        if len >= writable_len {
            // overflows block or buf.len() == writable_len
            writable_area.clone_from_slice(&buf[0..writable_len]); // fill block
            self.block_len += writable_len;
            self.message_len += writable_len as u64;
            self.process_block(); // perform hash calculation
            self.update(&buf[writable_len..]); // recursively write remaining
        } else {
            // don't fill block
            let write_area = &mut self.current_block[self.block_len..self.block_len + len];
            write_area.clone_from_slice(&buf[..]);
            self.block_len += len;
            self.message_len += len as u64;
        }
        len
    }
    fn finish(&mut self) -> Self::Output {
        self.current_block[self.block_len] = 0x80;
        if self.block_len + 1 + 8 > SHA256_BLOCK_SIZE {
            // data||0x80||size(u64) overflows block
            self.process_block(); // perform hash calculation
        }
        let writable_area = &mut self.current_block[SHA256_BLOCK_SIZE - 8..SHA256_BLOCK_SIZE];
        let len_bits = self.message_len * 8;
        writable_area.clone_from_slice(&len_bits.to_be_bytes());
        self.block_len = SHA256_BLOCK_SIZE;
        self.process_block();
        let mut final_hash = [0u8; 32];
        for i in 0..8 {
            let word_area = &mut final_hash[i * 4..i * 4 + 4];
            word_area.clone_from_slice(&self.h[i].0.to_be_bytes());
        }
        return final_hash;
    }
    fn get_block_count(&self) -> usize {
        ((self.message_len - self.block_len as u64) / Self::BLOCK_SIZE as u64) as usize
    }
}
impl Resumable for Sha256 {
    fn pause(&self) -> HashState {
        let h: [u32; 8] = [
            self.h[0].0,
            self.h[1].0,
            self.h[2].0,
            self.h[3].0,
            self.h[4].0,
            self.h[5].0,
            self.h[6].0,
            self.h[7].0,
        ];
        HashState::Sha256(hash_state::Sha256HashState {
            h,
            message_len: self.message_len,
            block_len: self.block_len,
            current_block: self.current_block,
        })
    }
    fn resume(hash_state: HashState) -> Result<Self, hash_state::Error> {
        match hash_state {
            HashState::Sha256(hs) => Ok(Self {
                h: arr32![hs.h[0], hs.h[1], hs.h[2], hs.h[3], hs.h[4], hs.h[5], hs.h[6], hs.h[7]],
                message_len: hs.message_len,
                block_len: hs.block_len,
                current_block: hs.current_block,
            }),
            _ => Err(hash_state::Error::HashTypeNotMatch),
        }
    }
}
impl Default for Sha256 {
    fn default() -> Self {
        Self {
            h: SHA256_H,
            current_block: [0u8; SHA256_BLOCK_SIZE],
            block_len: 0usize,
            message_len: 0u64,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;
    #[allow(non_upper_case_globals)]
    const data: &[u8] = &hex!("3082062130820509a0030201020204012d57ca300d06092a864886f70d01010b0500308182310b3009060355040613024a50310d300b060355040a0c044a504b4931253023060355040b0c1c4a504b4920666f7220757365722061757468656e7469636174696f6e313d303b060355040b0c344a6170616e204167656e637920666f72204c6f63616c20417574686f7269747920496e666f726d6174696f6e2053797374656d73301e170d3139303732353135323830365a170d3234303530323134353935395a302f310b3009060355040613024a503120301e06035504030c17383430343434453337504146484e30383232303030334130820122300d06092a864886f70d01010105000382010f003082010a0282010100c2e48c45c07363e246be44407c8af5317cbccd3aa8be5d26129224525ac9fd73bc65296102d48744600952f0493c397657c966e2564ff9ef5175357eec9628036096326107a90bd538f67390aaecbcd85672bdc66f088b3f1fa0657009c146dbec38111c50757358e3016803cf5ece665927b377afdf058432a624b372d2e39cf534ab9ed449da12ba239fe0dd96f65c72ccea6b6bfd9733c41e90edee1f842078ac5cde7c95c6242a322516ef22927f35abb8afe8327633d7ded0959384d205853b84726fabed29182f0213b6a74f118651d2c4c415b8253d3ac2d339c8775361b6201849fe99626f591f558c5c916a79182c856bb1599ad12be5d33748e7990203010001a38202ef308202eb300e0603551d0f0101ff04040302078030130603551d25040c300a06082b0601050507030230490603551d200101ff043f303d303b060b2a83088c9b55080501031e302c302a06082b06010505070201161e687474703a2f2f7777772e6a706b692e676f2e6a702f6370732e68746d6c3081b70603551d120481af3081aca481a93081a6310b3009060355040613024a5031273025060355040a0c1ee585ace79a84e5808be4babae8aa8de8a8bce382b5e383bce38393e382b931393037060355040b0c30e585ace79a84e5808be4babae8aa8de8a8bce382b5e383bce38393e382b9e588a9e794a8e88085e8a8bce6988ee794a831333031060355040b0c2ae59cb0e696b9e585ace585b1e59ba3e4bd93e68385e5a0b1e382b7e382b9e38386e383a0e6a99fe6a78b3081b10603551d1f0481a93081a63081a3a081a0a0819da4819a308197310b3009060355040613024a50310d300b060355040a0c044a504b4931253023060355040b0c1c4a504b4920666f7220757365722061757468656e7469636174696f6e3120301e060355040b0c1743524c20446973747269627574696f6e20506f696e747331143012060355040b0c0b49626172616b692d6b656e311a301806035504030c115473756b7562612d7368692043524c4450303a06082b06010505070101042e302c302a06082b06010505073001861e687474703a2f2f6f637370617574686e6f726d2e6a706b692e676f2e6a703081af0603551d230481a73081a480149567951b5ca70d84a0fff1d85a87f1aab1340385a18188a48185308182310b3009060355040613024a50310d300b060355040a0c044a504b4931253023060355040b0c1c4a504b4920666f7220757365722061757468656e7469636174696f6e313d303b060355040b0c344a6170616e204167656e637920666f72204c6f63616c20417574686f7269747920496e666f726d6174696f6e2053797374656d73820101301d0603551d0e0416041477f6c4d716d8cde22a27eed3d3af496e1fb0eff5300d06092a864886f70d01010b050003820101002addf5bce542900c6f93ab3ccfce694bc20fbf94d6096342c217cff14658047f4c1e40db2368267842081093b80a8a1cb9d0925efe110240a7115fb9831ecbb5f70e1fa38bb97842ad68204f411a938ac7fb316bb86dd0e32ea248d780bf8bf4e130dbf156a336ede2c0a1a52f4c46f25c59843973c19e910a11a72b802a55fe4a98d202003f287ab62f90bbf83f577c74a499561ee005ad9bed1056977a529a4f3c8cd395a37e7f5b3c9e7f98c113a091ab75525589e91dc5f152d35ad209f6c066c0b69bc1193b92c6eb8781d5cccbc353f6d521cc37af3cac600c61df67a7117c8dfc5b33446276e2cc0515e859bea1dfd37aa4c238e665f655d1b14f5fd3");
    #[test]
    fn it_hashes_hash_null() {
        let mut hasher = Sha256::new();
        let hash = hasher.finish();
        assert_eq!(
            hash,
            hex!("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
        )
    }
    #[test]
    fn it_hashes_hash_abc() {
        let mut hasher = Sha256::new();
        let written_bytes = hasher.update(b"abc");
        assert_eq!(written_bytes, 3);
        let hash = hasher.finish();
        assert_eq!(
            hash,
            hex!("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad")
        )
    }
    #[test]
    fn it_hashes_long_data() {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let hash = hasher.finish();
        assert_eq!(
            hash,
            hex!("449ef3f93be0c5425330af2b41c63769190aa75a713223d96839f6537104007e")
        )
    }
    #[test]
    fn it_hashes_splitted_data() {
        let mut hasher = Sha256::new();
        hasher.update(&data[0..64]);
        hasher.update(&data[64..345]);
        hasher.update(&data[345..356]);
        hasher.update(&data[356..356]);
        hasher.update(&data[356..357]);
        hasher.update(&data[357..]);
        let hash = hasher.finish();
        assert_eq!(
            hash,
            hex!("449ef3f93be0c5425330af2b41c63769190aa75a713223d96839f6537104007e")
        )
    }
    #[test]
    fn it_can_resume() {
        let mut hasher = Sha256::new();
        hasher.update(&data[0..195]);
        let state = hasher.pause();
        let mut hasher2 = Sha256::resume(state).unwrap();
        hasher2.update(&data[195..]);
        let hash = hasher2.finish();
        assert_eq!(
            hash,
            hex!("449ef3f93be0c5425330af2b41c63769190aa75a713223d96839f6537104007e")
        )
    }
}
