//! Minimal SHA-1 (FIPS 180-4), kept internal to avoid any runtime dependency.
//!
//! SHA-1 is used here only to derive a deterministic identifier, never for security.

const BLOCK_SIZE: usize = 64;
const INITIAL_STATE: [u32; 5] = [
    0x6745_2301,
    0xEFCD_AB89,
    0x98BA_DCFE,
    0x1032_5476,
    0xC3D2_E1F0,
];

pub(crate) fn sha1(message: &[u8]) -> [u8; 20] {
    let mut state = INITIAL_STATE;

    let mut blocks = message.chunks_exact(BLOCK_SIZE);
    for block in blocks.by_ref() {
        compress(&mut state, block);
    }

    let remainder = blocks.remainder();
    let bit_length = (message.len() as u64).wrapping_mul(8);
    let mut tail = [0u8; 2 * BLOCK_SIZE];
    tail[..remainder.len()].copy_from_slice(remainder);
    tail[remainder.len()] = 0x80;
    let tail_length = if remainder.len() < BLOCK_SIZE - 8 {
        BLOCK_SIZE
    } else {
        2 * BLOCK_SIZE
    };
    tail[tail_length - 8..tail_length].copy_from_slice(&bit_length.to_be_bytes());
    for block in tail[..tail_length].chunks_exact(BLOCK_SIZE) {
        compress(&mut state, block);
    }

    let mut digest = [0u8; 20];
    for (bytes, word) in digest.chunks_exact_mut(4).zip(state) {
        bytes.copy_from_slice(&word.to_be_bytes());
    }
    digest
}

fn compress(state: &mut [u32; 5], block: &[u8]) {
    let mut w = [0u32; 80];
    for (word, bytes) in w.iter_mut().zip(block.chunks_exact(4)) {
        *word = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    }
    for t in 16..80 {
        w[t] = (w[t - 3] ^ w[t - 8] ^ w[t - 14] ^ w[t - 16]).rotate_left(1);
    }

    let [mut a, mut b, mut c, mut d, mut e] = *state;
    for (t, &word) in w.iter().enumerate() {
        let (f, k) = match t {
            0..=19 => ((b & c) | (!b & d), 0x5A82_7999),
            20..=39 => (b ^ c ^ d, 0x6ED9_EBA1),
            40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1B_BCDC),
            _ => (b ^ c ^ d, 0xCA62_C1D6),
        };
        let temp = a
            .rotate_left(5)
            .wrapping_add(f)
            .wrapping_add(e)
            .wrapping_add(k)
            .wrapping_add(word);
        e = d;
        d = c;
        c = b.rotate_left(30);
        b = a;
        a = temp;
    }

    for (current, new) in state.iter_mut().zip([a, b, c, d, e]) {
        *current = current.wrapping_add(new);
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::sha1;
    use std::{string::String, vec};

    fn hex(digest: [u8; 20]) -> String {
        digest
            .iter()
            .map(|byte| std::format!("{byte:02x}"))
            .collect()
    }

    #[test]
    fn hashes_fips_180_test_vectors() {
        let vectors: [(&[u8], &str); 4] = [
            (b"", "da39a3ee5e6b4b0d3255bfef95601890afd80709"),
            (b"abc", "a9993e364706816aba3e25717850c26c9cd0d89d"),
            (
                b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
                "84983e441c3bd26ebaae4aa1f95129e5e54670f1",
            ),
            (
                b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu",
                "a49b2446a02c645bf419f995b67091253a04a259",
            ),
        ];
        for (message, expected) in vectors {
            assert_eq!(hex(sha1(message)), expected);
        }
    }

    #[test]
    fn hashes_one_million_a() {
        let message = vec![b'a'; 1_000_000];
        assert_eq!(
            hex(sha1(&message)),
            "34aa973cd4c4daa4f61eeb2bdbad27316534016f"
        );
    }

    #[test]
    fn hashes_messages_around_padding_boundaries() {
        // Lengths 55, 56 and 64 exercise the one-block vs two-block padding branches.
        assert_eq!(
            hex(sha1(&[b'a'; 55])),
            "c1c8bbdc22796e28c0e15163d20899b65621d65a"
        );
        assert_eq!(
            hex(sha1(&[b'a'; 56])),
            "c2db330f6083854c99d4b5bfb6e8f29f201be699"
        );
        assert_eq!(
            hex(sha1(&[b'a'; 64])),
            "0098ba824b5c16427bd7a1122a5a442a25ec644d"
        );
    }
}
