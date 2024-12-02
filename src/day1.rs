use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};

struct IdentityHasher(u64);

impl Hasher for IdentityHasher {
    #[inline(always)]
    fn finish(&self) -> u64 {
        self.0
    }

    // We are only using u32s
    fn write(&mut self, _bytes: &[u8]) {
        todo!();
    }

    #[inline(always)]
    fn write_u32(&mut self, i: u32) {
        self.0 = i as u64;
    }
}

impl Default for IdentityHasher {
    #[inline(always)]
    fn default() -> IdentityHasher {
        IdentityHasher(0)
    }
}

type IdentityBuildHasher = BuildHasherDefault<IdentityHasher>;

#[inline(always)]
fn dumb_parse(bytes: &[u8]) -> u32 {
    (bytes[0] - b'0') as u32 * 10_000
        + (bytes[1] - b'0') as u32 * 1_000
        + (bytes[2] - b'0') as u32 * 100
        + (bytes[3] - b'0') as u32 * 10
        + (bytes[4] - b'0') as u32
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    let bytes = input.as_bytes();

    let (mut vec1, mut vec2): (Vec<u32>, Vec<u32>) = bytes
        .chunks(14)
        .map(|l| (dumb_parse(&l[0..5]), dumb_parse(&l[8..13])))
        .unzip();

    vec1.sort_unstable();
    vec2.sort_unstable();

    vec1.into_iter()
        .zip(vec2.into_iter())
        .map(|(a, b)| a.abs_diff(b))
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    let bytes = input.as_bytes();
    let mut counts: HashMap<u32, u32, IdentityBuildHasher> =
        HashMap::with_capacity_and_hasher(input.len().div_ceil(13), IdentityBuildHasher::default());

    let vec: Vec<u32> = bytes
        .chunks(14)
        .map(|l| {
            let second = dumb_parse(&l[8..13]);
            counts.entry(second).and_modify(|v| *v += 1).or_insert(1);
            dumb_parse(&l[0..5])
        })
        .collect();

    vec.into_iter()
        .map(|i| counts.get(&i).map_or(0, |&v| v * i))
        .sum()
}
