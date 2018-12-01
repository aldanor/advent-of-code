pub fn parse_ints<'a>(input: &'a str) -> impl Iterator<Item = i64> + 'a {
    input
        .trim()
        .split_whitespace()
        .map(&str::parse::<i64>)
        .map(&Result::unwrap)
}
