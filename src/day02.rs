use std::collections::HashMap;

pub fn day02_1(input: &str) -> u32 {
    let mut map = HashMap::new();
    let (a, b) = input
        .trim()
        .split_whitespace()
        .map(|id| {
            map.clear();
            id.chars().for_each(|c| *map.entry(c).or_default() += 1);
            let (a, b) = map.values().fold((false, false), |acc, x| {
                (acc.0 || x % 2 == 0, acc.1 || x % 3 == 0)
            });
            (a as u32, b as u32)
        })
        .fold((0, 0), |acc, (a, b)| (acc.0 + a, acc.1 + b));
    a * b
}

pub fn day02_2(input: &str) -> String {
    let input = input.trim();
    for id1 in input.split_whitespace() {
        for id2 in input.split_whitespace() {
            let (i, n) =
                id1.chars()
                    .zip(id2.chars())
                    .enumerate()
                    .fold((0, 0), |acc, (i, (c1, c2))| {
                        let neq = c1 != c2;
                        (if neq { i } else { acc.0 }, acc.1 + neq as u32)
                    });
            if n == 1 {
                return format!("{}{}", &id1[..i], &id1[(i + 1)..]);
            }
        }
    }
    Default::default()
}

#[test]
fn test_day01_1() {
    const SAMPLE: &str = "abcdef bababc abbcde abcccd aabcdd abcdee ababab";
    assert_eq!(day02_1(SAMPLE), 12);
    const INPUT: &str = include_str!("../input/02.txt");
    assert_eq!(day02_1(INPUT), 6175);
}

#[test]
fn test_day02_1() {
    const SAMPLE: &str = "abcde fghij klmno pqrst fguij axcye wvxyz";
    assert_eq!(day02_2(SAMPLE), "fgij");
    const INPUT: &str = include_str!("../input/02.txt");
    assert_eq!(day02_2(INPUT), "asgwjcmzredihqoutcylvzinx");
}

#[cfg(feature = "bench")]
use test::{Bencher, black_box};

#[cfg_attr(feature = "bench", bench)]
#[cfg(feature = "bench")]
fn bench_day01_1(b: &mut Bencher) {
    const INPUT: &str = include_str!("../input/02.txt");
    b.iter(|| day02_1(black_box(INPUT)));
}

#[cfg_attr(feature = "bench", bench)]
#[cfg(feature = "bench")]
fn bench_day01_2(b: &mut Bencher) {
    const INPUT: &str = include_str!("../input/02.txt");
    b.iter(|| day02_2(black_box(INPUT)));
}
