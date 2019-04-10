use core::cmp;
use core::hash::{BuildHasher, Hasher};
use core::mem;
use core::ptr;

pub struct Murmur3Hasher {
    h1: u64,
    h2: u64,
    k1: u64,
    k2: u64,
    size: u64,
    have: usize,
}

#[derive(Clone, Default, Debug)]
pub struct Murmur3 {}

const C1: u64 = 0x87c37b91114253d5;
const C2: u64 = 0x4cf5ad432745937f;

macro_rules! load_int_le {
    ($buf: expr, $i: expr, $int_ty: ident) => ({
        debug_assert!($i + mem::size_of::<$int_ty>() <= $buf.len());
        let mut data = 0 as $int_ty;
        ptr::copy_nonoverlapping($buf.get_unchecked($i),
                                 &mut data as *mut _ as *mut u8,
                                 mem::size_of::<$int_ty>());
        data.to_le()
    });
}

#[inline]
unsafe fn u8to64_le(buf: &[u8], start: usize, len: usize) -> u64 {
    debug_assert!(len < 8);
    let mut i = 0; // current byte index (from LSB) in the output u64
    let mut out = 0;
    if i + 3 < len {
        out = load_int_le!(buf, start + i, u32) as u64;
        i += 4;
    }
    if i + 1 < len {
        out |= (load_int_le!(buf, start + i, u16) as u64) << (i * 8);
        i += 2
    }
    if i < len {
        out |= (*buf.get_unchecked(start + i) as u64) << (i * 8);
        i += 1;
    }
    debug_assert_eq!(i, len);
    out
}

#[inline]
fn fmix64(mut k: u64) -> u64 {
    k ^= k >> 33;
    k = k.wrapping_mul(0xff51afd7ed558ccd);
    k ^= k >> 33;
    k = k.wrapping_mul(0xc4ceb9fe1a85ec53);
    k ^= k >> 33;

    k
}

impl Murmur3Hasher {
    pub const fn new(seed: u32) -> Self {
        Self {
            h1: seed as u64,
            h2: seed as u64,
            k1: 0,
            k2: 0,
            size: 0,
            have: 0,
        }
    }

    pub fn finalize(&self) -> (u64, u64) {
        let mut h1 = self.h1;
        let mut h2 = self.h2;
        if self.have != 0 {
            if self.have > 8 {
                let mut k2 = self.k2;
                k2 = k2.wrapping_mul(C2); // k2 *= C2;
                k2 = k2.rotate_left(33); // ROTL64(k2,33);
                k2 = k2.wrapping_mul(C1); // k2 *= C1;
                h2 ^= k2;
            }
            let mut k1 = self.k1;
            k1 = k1.wrapping_mul(C1); // k1 *= C1;
            k1 = k1.rotate_left(31); // ROTL64(k1,31);
            k1 = k1.wrapping_mul(C2); // k1 *= C2;
            h1 ^= k1;
        }

        let size = self.size + (self.have as u64);
        //println!("size: {}", size);
        h1 ^= size;
        h2 ^= size;

        h1 = h1.wrapping_add(h2); // h1 += h2;
        h2 = h2.wrapping_add(h1); // h2 += h1;

        h1 = fmix64(h1);
        h2 = fmix64(h2);

        h1 = h1.wrapping_add(h2); // h1 += h2;
        h2 = h2.wrapping_add(h1); // h2 += h1;

        (h1, h2)
    }
}

impl Hasher for Murmur3Hasher {
    fn write(&mut self, bytes: &[u8]) {
        let mut offset = 0;
        let mut remain = bytes.len();

        if self.have < 8 {
            if self.have == 0 && remain >= 8 {
                self.k1 = unsafe { load_int_le!(bytes, offset, u64) };
                self.have += 8;
                offset += 8;
                remain -= 8;
            } else {
                let need = 8 - self.have;
                let take = cmp::min(need, remain);
                self.k1 |= unsafe { u8to64_le(bytes, offset, take) } << 8 * self.have;
                self.have += take;
                offset += take;
                remain -= take;
            }
        }
        if self.have >= 8 && self.have < 16 {
            if self.have == 8 && remain >= 8 {
                self.k2 = unsafe { load_int_le!(bytes, offset, u64) };
                self.have += 8;
                offset += 8;
                remain -= 8;
            } else {
                let need = 16 - self.have;
                let take = cmp::min(need, remain);
                self.k2 |= unsafe { u8to64_le(bytes, offset, take) } << 8 * (self.have - 8);
                self.have += take;
                offset += take;
                remain -= take;
            }
        }
        if self.have == 16 {
            loop {
                self.k1 = self.k1.wrapping_mul(C1); // k1 *= C1;
                self.k1 = self.k1.rotate_left(31); // ROTL64(k1,31)
                self.k1 = self.k1.wrapping_mul(C2); // k1 *= C2;
                self.h1 ^= self.k1;

                self.h1 = self.h1.rotate_left(27); // ROTL64(h1,27)
                self.h1 = self.h1.wrapping_add(self.h2); // h1 += h2;
                self.h1 = self.h1.wrapping_mul(5).wrapping_add(0x52dce729); // h1 = h1*5+0x52dce729;

                self.k2 = self.k2.wrapping_mul(C2); // k2 *= C2;
                self.k2 = self.k2.rotate_left(33); // ROTL64(k2,33)
                self.k2 = self.k2.wrapping_mul(C1); // k2 *= C1;
                self.h2 ^= self.k2;

                self.h2 = self.h2.rotate_left(31); // ROTL64(h2,31)
                self.h2 = self.h2.wrapping_add(self.h1); // h2 += h1;
                self.h2 = self.h2.wrapping_mul(5).wrapping_add(0x38495ab5); // h2 = h2*5+0x38495ab5;

                self.size += 16;
                if remain >= 16 {
                    self.k1 = unsafe { load_int_le!(bytes, offset, u64) };
                    offset += 8;
                    self.k2 = unsafe { load_int_le!(bytes, offset, u64) };
                    offset += 8;
                    remain -= 16;
                } else {
                    break;
                }
            }

            //println!("remain2: {}", remain);
            if remain >= 8 {
                self.k1 = unsafe { load_int_le!(bytes, offset, u64) };
                if remain > 8 {
                    self.k2 = unsafe { u8to64_le(bytes, offset + 8, remain - 8) };
                } else {
                    self.k2 = 0;
                }
            } else if remain > 0 {
                self.k1 = unsafe { u8to64_le(bytes, offset, remain) };
                self.k2 = 0;
            } else {
                self.k1 = 0;
                self.k2 = 0;
            }
            self.have = remain;
        }
    }

    fn finish(&self) -> u64 {
        self.finalize().0
    }
}

impl Murmur3 {
    pub const fn new() -> Self {
        Self {}
    }
}

impl BuildHasher for Murmur3 {
    type Hasher = Murmur3Hasher;

    #[inline]
    fn build_hasher(&self) -> Murmur3Hasher {
        Murmur3Hasher::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_hashes(data: &[u8], seed: u32, h1: u64, h2: u64) {
        let n = data.len();
        for i in 0..(n+1) {
            let mut hasher = Murmur3Hasher::new(seed);
            hasher.write(&data[0..i]);
            hasher.write(&data[i..n]);
            let (x1, x2) = hasher.finalize();
            assert_eq!((x1, x2), (h1, h2));
        }
    }

    #[test]
    fn test_vectors() {
        assert_hashes("".as_bytes(), 0, 0, 0);
        assert_hashes("a".as_bytes(), 0, 0x85555565f6597889, 0xe6b53a48510e895a);
        assert_hashes("ab".as_bytes(), 0, 0x938b11ea16ed1b2e, 0xe65ea7019b52d4ad);
        assert_hashes("abc".as_bytes(), 0, 0xb4963f3f3fad7867, 0x3ba2744126ca2d52);
        assert_hashes("abcd".as_bytes(), 0, 0xb87bb7d64656cd4f, 0xf2003e886073e875);
        assert_hashes("abcde".as_bytes(), 0, 0x2036d091f496bbb8, 0xc5c7eea04bcfec8c);
        assert_hashes("abcdef".as_bytes(), 0, 0xe47d86bfaca3bf55, 0xb07109993321845c);
        assert_hashes("abcdefg".as_bytes(), 0, 0xa6cd2f9fc09ee499, 0x1c3aa23ab155bbb6);
        assert_hashes("abcdefgh".as_bytes(), 0, 0xcc8a0ab037ef8c02, 0x48890d60eb6940a1);
        assert_hashes("abcdefghi".as_bytes(), 0, 0x0547c0cff13c7964, 0x79b53df5b741e033);
        assert_hashes("abcdefghij".as_bytes(), 0, 0xb6c15b0d772f8c99, 0xa24d85dc8c651ac9);
        assert_hashes("abcdefghijk".as_bytes(), 0, 0xa895d0b8df789d02, 0xbb7c31e2455ae771);
        assert_hashes("abcdefghijkl".as_bytes(), 0, 0x8ef39bb1e67ae194, 0x1f9e303272ff621c);
        assert_hashes("abcdefghijklm".as_bytes(), 0, 0x1648288da7c0fa73, 0x2e657bff0de7cc7f);
        assert_hashes("abcdefghijklmn".as_bytes(), 0, 0x91d094a7f5c375e0, 0xee096027d26a3324);
        assert_hashes("abcdefghijklmno".as_bytes(), 0, 0x8abe2451890c2ffb, 0x6a548c2d9c962a61);
        assert_hashes("abcdefghijklmnop".as_bytes(), 0, 0xc4ca3ca3224cb723, 0x4333d695b331eb1a);
        assert_hashes("abcdefghijklmnopqrstuvwxyz".as_bytes(), 0, 0x749c9d7e516f4aa9, 0xe9ad9c89b6a7d529);
        assert_hashes("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".as_bytes(), 0, 0x7a5bcce072ef9a8a, 0xcca67f5136a9c57f);
        assert_hashes("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".as_bytes(), 0, 0x49991f325fd73e3b, 0xcbadd23ca9ceb9bc);
        assert_hashes("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes(), 0, 0x1a88e52f0752fee5, 0x76e327368cf3ee7c);

        assert_hashes("The quick brown fox jumps over the lazy dog".as_bytes(), 0x9747b28c, 0x738a7f3bd2633121, 0xf94573727ec016e5);
        assert_hashes("The quick brown fox jumps over the lazy cog".as_bytes(), 0x9747b28c, 0xb8cd57b070826194, 0x556f455b5873f83c);
        assert_hashes("THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG".as_bytes(), 0x9747b28c, 0x788990e327d08a6c, 0xcfa49c7867cbd28a);
        assert_hashes("THE QUICK BROWN FOX JUMPS OVER THE LAZY COG".as_bytes(), 0x9747b28c, 0x5efbd2529a4d90dd, 0xa0be246654d0ea71);
        assert_hashes("the quick brown fox jumps over the lazy dog".as_bytes(), 0x9747b28c, 0xcd212cbd5168faa8, 0xd0748b96c8803ef3);
        assert_hashes("the quick brown fox jumps over the lazy cog".as_bytes(), 0x9747b28c, 0x27aa16dd5a9a4c71, 0xe02bfd8321a7901f);

        assert_hashes("The quick brown fox jumps over the lazy dog".as_bytes(), 0, 0xe34bbc7bbc071b6c, 0x7a433ca9c49a9347);
        assert_hashes("The quick brown fox jumps over the lazy cog".as_bytes(), 0, 0x658ca970ff85269a, 0x43fee3eaa68e5c3e);
        assert_hashes("THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG".as_bytes(), 0, 0xa6201801320afbc9, 0x82396cb0607a3c4b);
        assert_hashes("THE QUICK BROWN FOX JUMPS OVER THE LAZY COG".as_bytes(), 0, 0x6a766cc894e6b024, 0x0b01bb9244d48f6c);
        assert_hashes("the quick brown fox jumps over the lazy dog".as_bytes(), 0, 0xbce4e9fee2ad86b3, 0x0ae2e374406e4b7f);
        assert_hashes("the quick brown fox jumps over the lazy cog".as_bytes(), 0, 0x2f09fe5672502232, 0x86758d1ebb24d124);

        assert_hashes("The quick brown fox jumps over the lazy dog".as_bytes(), 0xc58f1a7b, 0xac1f40eed20c9dff, 0x38935c52deeff526);
        assert_hashes("The quick brown fox jumps over the lazy cog".as_bytes(), 0xc58f1a7b, 0xf93938845b5c938c, 0xcdbc8bd57a4fb264);
        assert_hashes("THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG".as_bytes(), 0xc58f1a7b, 0xf249f1d9f383e469, 0x34fe27053ace80f4);
        assert_hashes("THE QUICK BROWN FOX JUMPS OVER THE LAZY COG".as_bytes(), 0xc58f1a7b, 0x081f1cba1cb41bd8, 0x6ff1e44e62a8813e);
        assert_hashes("the quick brown fox jumps over the lazy dog".as_bytes(), 0xc58f1a7b, 0x88ec96021b8af702, 0x640843c82e69c55c);
        assert_hashes("the quick brown fox jumps over the lazy cog".as_bytes(), 0xc58f1a7b, 0x9219b4b672765148, 0x81736d9f9f008440);
    }
}
