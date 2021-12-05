mod day1;
mod day2;
mod day3;
mod day4;
mod util;
fn main() {
    day1::part1(util::get_from_file(1));
    day1::part2(util::get_from_file(1));

    day2::part1(util::get_from_file(2));
    day2::part2(util::get_from_file(2));
    day2::part2(util::get_test_from_file(2));

    day3::part1(util::get_test_from_file(3));
    day3::part1(util::get_from_file(3));
    day3::part2(util::get_test_from_file(3));
    day3::part2(util::get_from_file(3));

    day4::part1(util::get_test_from_file(4));
    // day4::part1(util::get_from_file(4));
    day4::part2(util::get_test_from_file(4));
    // day4::part2(util::get_from_file(4));
}
