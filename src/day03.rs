use std::cmp::{Ordering, PartialOrd};
use std::iter;
use std::str::FromStr;

use itertools::Itertools;
use lazy_static::lazy_static;
use num_bigint::BigUint;
use num_traits::Zero;
use regex::Regex;

pub type Coord = usize;

pub const N: usize = 1024;

#[derive(Copy, Clone, Default, PartialEq, Eq)]
struct Rect {
    id: u32,
    x1: Coord,
    y1: Coord,
    x2: Coord,
    y2: Coord,
}

impl PartialOrd for Rect {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((self.x1 + self.y1).cmp(&(other.x1 + other.y1)))
    }
}

impl Ord for Rect {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Rect {
    pub fn parse(s: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
        }
        let caps = RE.captures(s).unwrap();
        let id = u32::from_str(&caps[1]).unwrap();
        let x1 = Coord::from_str(&caps[2]).unwrap();
        let y1 = Coord::from_str(&caps[3]).unwrap();
        let w = Coord::from_str(&caps[4]).unwrap();
        let h = Coord::from_str(&caps[5]).unwrap();
        let (x2, y2) = (x1 + w, y1 + h);
        Self { id, x1, y1, x2, y2 }
    }

    pub fn parse_many(s: &str) -> Vec<Self> {
        s.trim().lines().map(|s| Self::parse(s)).collect()
    }

    pub fn overlaps(&self, other: &Rect) -> bool {
        !(self.x1 >= other.x2 || other.x1 >= self.x2 || self.y1 >= other.y2 || other.y1 >= self.y2)
    }
}

fn make_board(rects: &[Rect]) -> Vec<BigUint> {
    let mut board = iter::repeat(BigUint::zero()).take(N).collect_vec();
    for rect in rects {
        let mut mask = [0u8; N];
        for x in rect.x1..rect.x2 {
            mask[x] = 1;
        }
        let mask = BigUint::from_bytes_be(&mask);
        for y in rect.y1..rect.y2 {
            board[y] += &mask;
        }
    }
    board
}

pub fn day03_1(input: &str) -> u32 {
    make_board(&Rect::parse_many(input))
        .into_iter()
        .flat_map(|x| x.to_bytes_be())
        .map(|x| (x > 1) as u32)
        .sum()
}

pub fn day03_2(input: &str) -> u32 {
    let mut rects = Rect::parse_many(input);
    rects.sort();
    let n = rects.len();
    let mut ok = iter::repeat(true).take(n).collect_vec();
    for i in 0..n {
        if !ok[i] {
            continue;
        }
        let ri = &rects[i];
        let min = ri.x1 + ri.y1;
        let max = ri.x2 + ri.y2;
        for j in 0..n {
            let rj = &rects[j];
            if rj.x2 + rj.y2 <= min {
                continue;
            }
            if rj.x1 + rj.y1 >= max {
                break;
            }
            if i != j && ri.overlaps(rj) {
                ok[i] = false;
                ok[j] = false;
            }
        }
        if ok[i] {
            return ri.id;
        }
    }
    0
}

#[test]
fn test_day03_1() {
    const SAMPLE: &str = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2";
    assert_eq!(day03_1(SAMPLE), 4);
    const INPUT: &str = include_str!("../input/03.txt");
    assert_eq!(day03_1(INPUT), 120408);
}

#[test]
fn test_day03_2() {
    const SAMPLE: &str = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2";
    assert_eq!(day03_2(SAMPLE), 3);
    const INPUT: &str = include_str!("../input/03.txt");
    assert_eq!(day03_2(INPUT), 1276);
}

#[cfg(feature = "bench")]
use test::{black_box, Bencher};

#[cfg_attr(feature = "bench", bench)]
#[cfg(feature = "bench")]
fn bench_day03_1(b: &mut Bencher) {
    const INPUT: &str = include_str!("../input/03.txt");
    b.iter(|| day03_1(black_box(INPUT)));
}

#[cfg_attr(feature = "bench", bench)]
#[cfg(feature = "bench")]
fn bench_day03_2(b: &mut Bencher) {
    const INPUT: &str = include_str!("../input/03.txt");
    b.iter(|| day03_2(black_box(INPUT)));
}
