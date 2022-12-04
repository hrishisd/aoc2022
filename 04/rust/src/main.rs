use scan_fmt::scan_fmt;

#[derive(Debug, Clone, Copy)]
struct Range {
    l: i32,
    r: i32,
}

fn main() {
    let assignments: Vec<(Range, Range)> = parse(include_str!("../../input"));
    println!("part 1: {}", part1(&assignments));
    println!("part 2: {}", part2(&assignments));
}

fn part1(assignments: &[(Range, Range)]) -> usize {
    count_matching(assignments, |r1, r2| r1.contains(r2) || r2.contains(r1))
}

fn part2(assignments: &[(Range, Range)]) -> usize {
    count_matching(assignments, |r1, r2| r1.overlaps(r2))
}

fn count_matching(assignments: &[(Range, Range)], pred: fn(Range, Range) -> bool) -> usize {
    assignments.iter().filter(|(r1, r2)| pred(*r1, *r2)).count()
}

fn parse(input: &str) -> Vec<(Range, Range)> {
    input
        .lines()
        .map(|line| {
            let (l1, r1, l2, r2) = scan_fmt!(line, "{}-{},{}-{}", i32, i32, i32, i32).unwrap();
            (Range { l: l1, r: r1 }, Range { l: l2, r: r2 })
        })
        .collect()
}

impl Range {
    fn contains(&self, other: Range) -> bool {
        self.l <= other.l && self.r >= other.r
    }

    fn overlaps(&self, other: Range) -> bool {
        !(self.r < other.l || self.l > other.r)
    }
}
