pub fn parse_ints<'a>(input: &'a str) -> impl Iterator<Item = i32> + 'a {
    input
        .trim()
        .split_whitespace()
        .map(&str::parse::<i32>)
        .map(&Result::unwrap)
}
