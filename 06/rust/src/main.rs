#![feature(array_windows)]
use fixedbitset::FixedBitSet;

fn main() {
    let buffer = include_str!("../../input").as_bytes();
    println!("part 1: {}", part1(buffer));
    println!("part 2: {}", part2(buffer));
}

fn part1(buffer: &[u8]) -> usize {
    key_idx::<4>(buffer)
}

fn part2(buffer: &[u8]) -> usize {
    key_idx::<14>(buffer)
}

fn key_idx<const N: usize>(buffer: &[u8]) -> usize {
    FixedBitSet::with_capacity(26);
    buffer
        .array_windows::<N>()
        .enumerate()
        .find(|(_, window)| unique::<N>(window))
        .map(|(idx, _)| idx + N)
        .expect("unable to find marker in buffer")
}

fn unique<const N: usize>(chars: &[u8; N]) -> bool {
    let mut set = FixedBitSet::with_capacity(26);
    for ele in chars {
        set.insert((ele - b'a') as usize);
    }
    set.count_ones(..) == N
}
