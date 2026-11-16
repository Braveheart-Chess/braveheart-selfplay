//! The random number generator self-play draws from.
//!
//! Copied verbatim from the engine's `openings::Rng` (`braveheart-tools`,
//! v0.54.0) when this crate moved out of the engine repository. Every shard's
//! manifest records the seed it was generated from, so this stream is part of
//! the data's reproducibility: it must never change. The test below pins it.

/// A small, fast, seeded xorshift generator: reproducible streams for
/// openings and self-play, no dependency.
pub struct Rng(u64);

impl Rng {
    /// Seeded through splitmix64 so that nearby seeds give unrelated streams
    /// and no seed yields the all-zero state xorshift cannot leave.
    pub fn seeded(seed: u64) -> Rng {
        let mut z = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        Rng((z ^ (z >> 31)) | 1)
    }

    /// The next number in the stream.
    pub fn next_u64(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Rng;

    /// The first numbers of three streams, computed from the engine's own
    /// implementation when it was copied. A change here breaks the
    /// reproducibility of every recorded shard.
    #[test]
    fn the_stream_is_the_engines() {
        let pinned: [(u64, [u64; 4]); 3] = [
            (
                0,
                [
                    7377219508542733812,
                    3375351177031125519,
                    1405982755453415387,
                    17396837829720340804,
                ],
            ),
            (
                42,
                [
                    18108192690585582856,
                    6830302529404445810,
                    7514410519785295290,
                    9589382499465979553,
                ],
            ),
            (
                11400714781115297274,
                [
                    12821202192682511110,
                    15085923168601666384,
                    16912066344925924278,
                    6901823892363831841,
                ],
            ),
        ];
        for (seed, expected) in pinned {
            let mut rng = Rng::seeded(seed);
            let got = [
                rng.next_u64(),
                rng.next_u64(),
                rng.next_u64(),
                rng.next_u64(),
            ];
            assert_eq!(got, expected, "seed {seed}");
        }
    }
}
