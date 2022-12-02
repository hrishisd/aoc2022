fn main() {
    let mut totals: Vec<i32> = include_str!("../../input")
        .trim()
        .split("\n\n")
        .map(|group| {
            group
                .split('\n')
                .map(|s| s.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect();
    totals.sort_by_key(|i| -i);

    println!("part1: {}", totals[0]);
    println!("part2: {}", totals[0] + totals[1] + totals[2]);
}
