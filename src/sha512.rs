use crate::consts::*;
use crate::hash_state;
use crate::hash_state::HashState;
use crate::traits::*;
/// Calculates SHA-512
pub struct Sha512 {
    /// Hash values
    h: [u64; 8],
    /// The max length of message (in bytes) defined in fips 180-4
    message_len: u128,
    /// The length of `current_block` in bytes
    block_len: usize,
    /// The incomplete block that is waiting to be filled and hashed
    current_block: [u8; SHA512_BLOCK_SIZE],
}

impl Sha512 {
    /// Create new instance
    pub fn new() -> Self {
        Self {
            h: SHA512_H,
            current_block: [0u8; SHA512_BLOCK_SIZE],
            block_len: 0usize,
            message_len: 0u128,
        }
    }
    /// Compute hash for current block
    fn process_block(&mut self) {
        let (a, b, c, d, e, f, g, h) = self.process_block_parts();

        self.h[0] = self.h[0].wrapping_add(a);
        self.h[1] = self.h[1].wrapping_add(b);
        self.h[2] = self.h[2].wrapping_add(c);
        self.h[3] = self.h[3].wrapping_add(d);
        self.h[4] = self.h[4].wrapping_add(e);
        self.h[5] = self.h[5].wrapping_add(f);
        self.h[6] = self.h[6].wrapping_add(g);
        self.h[7] = self.h[7].wrapping_add(h);

        self.current_block = [0u8; SHA512_BLOCK_SIZE]; // next block
        self.block_len = 0; // reset block
    }

    /// Conbines 8 byte and returns as u64.
    const fn get_word64_in_block(&self, i: usize) -> u64 {
        ((self.current_block[i * 8] as u64) << 56)
            | ((self.current_block[i * 8 + 1] as u64) << 48)
            | ((self.current_block[i * 8 + 2] as u64) << 40)
            | ((self.current_block[i * 8 + 3] as u64) << 32)
            | ((self.current_block[i * 8 + 4] as u64) << 24)
            | ((self.current_block[i * 8 + 5] as u64) << 16)
            | ((self.current_block[i * 8 + 6] as u64) << 8)
            | (self.current_block[i * 8 + 7] as u64)
    }
    const fn process_block_parts(&self) -> (u64, u64, u64, u64, u64, u64, u64, u64) {
        if self.block_len != SHA512_BLOCK_SIZE {
            panic!("block is not filled");
        }
        let mut w = [0_u64; 80];

        let mut t = 0;
        while t < 16 {
            w[t] = self.get_word64_in_block(t);
            t += 1;
        }
        while t < 80 {
            w[t] = lsigma1(w[t - 2])
                .wrapping_add(w[t - 7])
                .wrapping_add(lsigma0(w[t - 15]))
                .wrapping_add(w[t - 16]);
            t += 1;
        }
        let mut a = self.h[0];
        let mut b = self.h[1];
        let mut c = self.h[2];
        let mut d = self.h[3];
        let mut e = self.h[4];
        let mut f = self.h[5];
        let mut g = self.h[6];
        let mut h = self.h[7];

        let mut t = 0;
        while t < 80 {
            let t1 = h
                .wrapping_add(sigma1(e))
                .wrapping_add(ch(e, f, g))
                .wrapping_add(SHA512_K[t])
                .wrapping_add(w[t]);
            let t2 = sigma0(a).wrapping_add(maj(a, b, c));
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);

            t += 1;
        }

        (a, b, c, d, e, f, g, h)
    }
}
const fn rotr(x: u64, n: usize) -> u64 {
    (x >> n) | (x << (64 - n))
}
const fn sigma0(x: u64) -> u64 {
    rotr(x, 28) ^ rotr(x, 34) ^ rotr(x, 39)
}
const fn sigma1(x: u64) -> u64 {
    rotr(x, 14) ^ rotr(x, 18) ^ rotr(x, 41)
}
const fn lsigma0(x: u64) -> u64 {
    rotr(x, 1) ^ rotr(x, 8) ^ (x >> 7)
}
const fn lsigma1(x: u64) -> u64 {
    rotr(x, 19) ^ rotr(x, 61) ^ (x >> 6)
}
const fn ch(x: u64, y: u64, z: u64) -> u64 {
    (x & y) ^ (!x & z)
}
const fn maj(x: u64, y: u64, z: u64) -> u64 {
    (x & y) ^ (x & z) ^ (y & z)
}
impl StreamHasher for Sha512 {
    type Output = [u8; 64];
    const BLOCK_SIZE: usize = SHA512_BLOCK_SIZE;
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
            self.message_len += writable_len as u128;
            self.process_block(); // perform hash calculation
            self.update(&buf[writable_len..]); // recursively write remaining
        } else {
            // don't fill block
            let write_area = &mut self.current_block[self.block_len..self.block_len + len];
            write_area.clone_from_slice(buf);
            self.block_len += len;
            self.message_len += len as u128;
        }
        len
    }
    fn finish(mut self) -> Self::Output {
        self.current_block[self.block_len] = 0x80;
        if self.block_len + 1 + 16 > Self::BLOCK_SIZE {
            // data||0x80||size(u128) overflows block

            self.block_len = Self::BLOCK_SIZE;
            self.process_block(); // perform hash calculation
        }
        let writable_area = &mut self.current_block[Self::BLOCK_SIZE - 16..Self::BLOCK_SIZE];
        let len_bits = self.message_len * 8;
        writable_area.clone_from_slice(&len_bits.to_be_bytes());
        self.block_len = Self::BLOCK_SIZE;
        self.process_block();
        let mut final_hash: Self::Output = [0; 64];
        for i in 0..8 {
            let word_area = &mut final_hash[i * 8..i * 8 + 8];
            word_area.clone_from_slice(&self.h[i].to_be_bytes());
        }
        final_hash
    }
}
impl Resumable for Sha512 {
    fn pause(self) -> HashState {
        let h: [u64; 8] = [
            self.h[0], self.h[1], self.h[2], self.h[3], self.h[4], self.h[5], self.h[6], self.h[7],
        ];
        HashState::Sha512(hash_state::Sha512HashState {
            h,
            message_len: self.message_len,
            block_len: self.block_len,
            current_block: self.current_block,
        })
    }
    fn resume(hash_state: HashState) -> Result<Self, hash_state::Error> {
        match hash_state {
            HashState::Sha512(hs) => Ok(Self {
                h: [
                    hs.h[0], hs.h[1], hs.h[2], hs.h[3], hs.h[4], hs.h[5], hs.h[6], hs.h[7],
                ],
                message_len: hs.message_len,
                block_len: hs.block_len,
                current_block: hs.current_block,
            }),
            _ => Err(hash_state::Error::HashTypeNotMatch),
        }
    }
}
impl Default for Sha512 {
    fn default() -> Self {
        Self::new()
    }
}
