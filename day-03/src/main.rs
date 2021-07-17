use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

use itertools::Itertools; // cartesian_product()
use recap::Recap; // #[recap()]
use serde::Deserialize; // #[derive(Deserialize)]

// Puzzle data type
#[derive(Deserialize, Recap)]
#[recap(
    regex = r#"#(?P<id>\d+) @ (?P<x>\d+),(?P<y>\d+): (?P<w>\d+)x(?P<h>\d+)"#
)]
struct Claim {
    id: i32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

// ----------------------------------------------------------------------------

fn main() -> io::Result<()> {
    let claims: Vec<Claim> = io::stdin()
        .lock()
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| l.parse().ok())
        .collect();

    // Part 1
    // ------

    // Tests
    #[rustfmt::skip]
    assert_eq!(
        part1(&[
            Claim { id: 1, x: 1, y: 3, w: 4, h: 4 },
            Claim { id: 2, x: 3, y: 1, w: 4, h: 4 },
            Claim { id: 3, x: 5, y: 5, w: 2, h: 2 },
        ]),
        4
    );

    // Puzzle answer
    let answer1 = part1(&claims);
    println!("Part 1: {}", answer1);
    assert_eq!(answer1, 115242);

    // Part 2
    // ------

    // Tests
    #[rustfmt::skip]
    assert_eq!(
        part2(&[
            Claim { id: 1, x: 1, y: 3, w: 4, h: 4 },
            Claim { id: 2, x: 3, y: 1, w: 4, h: 4 },
            Claim { id: 3, x: 5, y: 5, w: 2, h: 2 },
        ]),
        3
  );

    // Puzzle answer
    let answer2 = part2(&claims);
    println!("Part 2: {}", answer2);
    assert_eq!(answer2, 1046);

    Ok(())
}

// ----------------------------------------------------------------------------

fn part1(claims: &[Claim]) -> usize {
    claims
        .iter()
        // Create a map of coordinates with their claim count:
        // HashMap<(i, j), count>
        .fold(HashMap::new(), |mut coord_count, claim| {
            (claim.x..(claim.x + claim.w))
                .cartesian_product(claim.y..(claim.y + claim.h))
                .for_each(|coord| {
                    // For each coordinate (i, j), store the times it gets claimed
                    *coord_count.entry(coord).or_insert(0) += 1;
                });
            coord_count
        })
        // Work only with the claim counts from the map
        .values()
        .filter(|&&count| count > 1)
        .count()
}

// ----------------------------------------------------------------------------

fn part2(claims: &[Claim]) -> i32 {
    let mut coord_ids = HashMap::new();
    let mut clean_claims = HashSet::new();

    for claim in claims.iter() {
        let mut unique = true;
        for coord in (claim.x..(claim.x + claim.w))
            .cartesian_product(claim.y..(claim.y + claim.h))
        {
            match coord_ids.entry(coord) {
                Entry::Occupied(e) => {
                    unique = false;
                    clean_claims.remove(e.get());
                    clean_claims.remove(&claim.id);
                }
                Entry::Vacant(e) => {
                    e.insert(claim.id);
                }
            };
        }

        if unique {
            clean_claims.insert(claim.id);
        }
    }

    *clean_claims.iter().next().unwrap_or(&0)
}

// ----------------------------------------------------------------------------
