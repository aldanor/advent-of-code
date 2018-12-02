use itertools::Itertools;
use num_integer::Integer;

use crate::util::*;

pub fn day01_1(input: &str) -> i32 {
    parse_ints(input).sum()
}

pub fn day01_2(input: &str) -> i32 {
    let s = parse_ints(input)
        .scan(0, |acc, x| {
            *acc += x;
            Some(*acc)
        })
        .collect_vec();
    let sum = *s.last().unwrap();
    let n = s.len();
    if sum == 0 {
        for j in 1..n {
            for i in 0..(j - 1) {
                if s[i] == s[j] {
                    return s[i];
                }
            }
        }
        return 0;
    }
    let mut out = (i32::max_value(), 0);
    for i in 0..(n - 1) {
        let si = unsafe { *s.get_unchecked(i) };
        for j in (i + 1)..n {
            let sj = unsafe { *s.get_unchecked(j) };
            let (d, r) = (si - sj).div_rem(&sum);
            if r == 0 {
                let dn = d * (n as i32);
                let (idx, val) = if d > 0 {
                    (dn + (j as i32), si)
                } else {
                    (-dn + (i as i32), sj)
                };
                if idx < out.0 {
                    out = (idx, val);
                }
            }
        }
    }
    out.1
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
