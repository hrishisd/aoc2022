use fixedbitset::FixedBitSet;

type Item = u8;
struct Sack(fixedbitset::FixedBitSet);

fn main() {
    let input = include_str!("../../input");
    let rucksacks: Vec<&[Item]> = input.lines().map(|line| line.as_bytes()).collect();
    println!("part 1: {}", part1(&rucksacks));
    println!("part 2: {}", part2(&rucksacks));
}

fn part1(rucksacks: &[&[Item]]) -> usize {
    rucksacks
        .iter()
        .map(|items| {
            let (left, right) = items.split_at(items.len() / 2);
            let mut left = Sack::from(left);
            let right = Sack::from(right);
            left.retain_common(right);
            left.get_remaining_item_priority()
        })
        .sum()
}

fn part2(rucksacks: &[&[Item]]) -> usize {
    rucksacks
        .chunks_exact(3)
        .map(|chunk| {
            let mut sack = Sack::from(chunk[0]);
            sack.retain_common(Sack::from(chunk[1]));
            sack.retain_common(Sack::from(chunk[2]));
            sack.get_remaining_item_priority()
        })
        .sum()
}

impl From<&[Item]> for Sack {
    fn from(items: &[Item]) -> Self {
        let priorities = items.iter().map(|item| priority(*item) as usize);
        let bitset = FixedBitSet::from_iter(priorities);
        Sack(bitset)
    }
}

impl Sack {
    fn retain_common(&mut self, other: Self) {
        self.0.intersect_with(&other.0);
    }

    fn get_remaining_item_priority(&self) -> usize {
        let mut ones = self.0.ones();
        let result = ones.next().unwrap();
        assert!(ones.next().is_none());
        result
    }
}

fn priority(item: Item) -> u8 {
    if item.is_ascii_lowercase() {
        1 + item - b'a'
    } else {
        27 + item - b'A'
    }
}
