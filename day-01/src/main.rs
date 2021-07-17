use std::collections::HashSet;
use std::io::{self, BufRead};

// Puzzle data type
type Change = i32;

// ----------------------------------------------------------------------------

fn main() -> io::Result<()> {
    let changes: Vec<Change> = io::stdin()
        .lock()
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| l.parse().ok())
        .collect();

    // Part 1
    // ------

    // Tests
    assert_eq!(part1(&[1, -2, 3, 1]), 3);
    assert_eq!(part1(&[1, 1, 1]), 3);
    assert_eq!(part1(&[1, 1, -2]), 0);
    assert_eq!(part1(&[-1, -2, -3]), -6);

    // Puzzle answer
    let answer1 = part1(&changes);
    println!("Part 1: {}", answer1);
    assert_eq!(answer1, 472);

    // Part 2
    // ------

    // Tests
    assert_eq!(part2(&[1, -2, 3, 1]), 2);
    assert_eq!(part2(&[1, -1]), 0);
    assert_eq!(part2(&[3, 3, 4, -2, -4]), 10);
    assert_eq!(part2(&[-6, 3, 8, 5, -6]), 5);
    assert_eq!(part2(&[7, 7, -2, -7, -4]), 14);

    // Puzzle answer
    let answer2 = part2(&changes);
    println!("Part 2: {}", answer2);
    assert_eq!(answer2, 66932);

    Ok(())
}

// ----------------------------------------------------------------------------

fn part1(changes: &[Change]) -> i32 {
    changes.iter().sum()
}

// ----------------------------------------------------------------------------

fn part2(changes: &[Change]) -> i32 {
    let mut sum: i32 = 0;
    let mut reached = HashSet::new();
    reached.insert(0);

    changes.iter().cycle().all(|change| {
        sum += change;
        reached.insert(sum)
    });
    sum
}

// ----------------------------------------------------------------------------
