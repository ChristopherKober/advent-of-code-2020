use aoc2020::setup::matches::get_matches;
use aoc2020::logic::router::route_puzzle;

fn main() {
    let matches = get_matches();

    let day = matches.value_of("day").unwrap().parse::<i32>().unwrap();
    let part = matches.value_of("part").unwrap().parse::<i32>().unwrap();
    let mut files = matches.values_of("file").unwrap_or_default();

    route_puzzle(day, part, &mut files)
}
