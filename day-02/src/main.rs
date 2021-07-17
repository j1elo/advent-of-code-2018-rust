use std::collections::{BTreeSet, HashMap, HashSet};
use std::io::{self, BufRead};

use itertools::Itertools; // tuple_windows()
use strsim; // hamming()

// Puzzle data type
type BoxId<'a> = &'a str;
// type BoxId = String;

// ----------------------------------------------------------------------------

fn main() -> io::Result<()> {
    let strings: Vec<String> =
        io::stdin().lock().lines().filter_map(|l| l.ok()).collect();
    let box_ids: Vec<BoxId> = strings.iter().map(|l| l.as_str()).collect();

    // Part 1
    // ------

    // Tests
    #[rustfmt::skip]
    assert_eq!(
        part1(&[
            "abcdef",
            "bababc",
            "abbcde",
            "abcccd",
            "aabcdd",
            "abcdee",
            "ababab"
        ]),
        12
    );

    // Puzzle answer
    let answer1 = part1(&box_ids[..]);
    println!("Part 1: {}", answer1);
    assert_eq!(answer1, 6916);

    // Part 2
    // ------

    // Tests
    #[rustfmt::skip]
    assert_eq!(
        part2(&[
            "abcde",
            "fghij",
            "klmno",
            "pqrst",
            "fguij",
            "axcye",
            "wvxyz"
        ]),
        "fgij"
    );

    // Puzzle answer
    let answer2 = part2(&box_ids);
    println!("Part 2: {}", answer2);
    assert_eq!(answer2, "oeylbtcxjqnzhgyylfapviusr");

    Ok(())
}

// ----------------------------------------------------------------------------

fn part1(box_ids: &[BoxId]) -> u32 {
    let mut twos: u32 = 0;
    let mut threes: u32 = 0;

    for box_id in box_ids {
        let counts: HashSet<u32> = box_id
            // For each char, use a map to count character appearances
            .chars()
            .fold(HashMap::new(), |mut count_map, ch| {
                *count_map.entry(ch).or_insert(0) += 1;
                count_map
            })
            // Convert the map into a set, so all duplicates go away
            .values()
            // Get rid of the references and make into a HashSet
            .cloned()
            .collect();

        // Check if the desired values exist in the set
        if counts.contains(&2) {
            twos += 1;
        }
        if counts.contains(&3) {
            threes += 1;
        }
    }

    twos * threes
}

// ----------------------------------------------------------------------------

fn part2(box_ids: &[BoxId]) -> String {
    box_ids
        .iter()
        .cloned()
        // Prepare a sorted set of IDs
        .collect::<BTreeSet<BoxId>>()
        // Iterate by pairs (windows size = 2), and get the first couple IDs
        // that differ only by 1 char (Hamming distance = 1)
        .iter()
        .tuple_windows()
        .filter_map(|(&a, &b)| match strsim::hamming(a, b) {
            Ok(1) => Some(
                // Go over all chars on both strings, and keep only those
                // that are the same; removes the char that changes
                a.chars()
                    .zip(b.chars())
                    .filter_map(|(a, b)| if a == b { Some(a) } else { None })
                    .collect::<String>(),
            ),
            _ => None,
        })
        // Advance the iterator to get the first match; makes the lazy
        // filter_map() to actually work over the IDs
        .next()
        .unwrap_or_default()
}

// ----------------------------------------------------------------------------
