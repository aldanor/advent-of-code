use hashbrown::HashSet;
use itertools::Itertools;

use crate::util::*;

pub fn day01_1(input: &str) -> i64 {
    parse_ints(input).sum()
}

pub fn day01_2(input: &str) -> i64 {
    let mut set = HashSet::new();
    set.insert(0);
    parse_ints(input)
        .collect_vec()
        .into_iter()
        .cycle()
        .scan(0, |acc, x| {
            *acc += x;
            Some(*acc)
        })
        .skip_while(|&x| set.insert(x))
        .next()
        .unwrap()
}

#[test]
fn test_day01_1() {
    assert_eq!(day01_1("+1 +1 +1"), 3);
    assert_eq!(day01_1("+1 +1 -2"), 0);
    assert_eq!(day01_1("-1 -2 -3"), -6);
    const INPUT: &str = include_str!("../input/01.txt");
    assert_eq!(day01_1(INPUT), 445);
}

#[test]
fn test_day01_2() {
    assert_eq!(day01_2("+1 -1"), 0);
    assert_eq!(day01_2("+3 +3 +4 -2 -4"), 10);
    assert_eq!(day01_2("-6 +3 +8 +5 -6"), 5);
    assert_eq!(day01_2("+7 +7 -2 -7 -4"), 14);
    const INPUT: &str = include_str!("../input/01.txt");
    assert_eq!(day01_2(INPUT), 219);
}

#[cfg(feature = "bench")]
use test::{black_box, Bencher};

#[cfg_attr(feature = "bench", bench)]
#[cfg(feature = "bench")]
fn bench_day01_1(b: &mut Bencher) {
    const INPUT: &str = include_str!("../input/01.txt");
    b.iter(|| day01_1(black_box(INPUT)));
}

#[cfg_attr(feature = "bench", bench)]
#[cfg(feature = "bench")]
fn bench_day01_2(b: &mut Bencher) {
    const INPUT: &str = include_str!("../input/01.txt");
    b.iter(|| day01_2(black_box(INPUT)));
}
