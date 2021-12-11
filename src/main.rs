mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod util;
fn main() {
    if false {
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
        day4::part1(util::get_from_file(4));
        day4::part2(util::get_from_file(4));

        day5::part1(util::get_test_from_file(5));
        day5::part1(util::get_from_file(5));
        day5::part2(util::get_from_file(5));

        day6::part1_test(util::get_test_from_file(6));
        day6::part1(util::get_from_file(6));
        day6::part2(util::get_from_file(6));

        day7::part1(util::get_test_from_file(7));
        day7::part1(util::get_from_file(7));
        day7::part2(util::get_from_file(7));

        day8::part1(util::get_test_from_file(8));
        day8::part1(util::get_from_file(8));
        day8::part2(util::get_test_from_file(88));
        day8::part2(util::get_from_file(8));

        day9::part1(util::get_test_from_file(9));
        day9::part1(util::get_from_file(9));
        day9::part2(util::get_test_from_file(9));
        day9::part2(util::get_from_file(9));

        day10::part1(util::get_test_from_file(10));
        day10::part1(util::get_from_file(10));
        day10::part2(util::get_test_from_file(10));
        day10::part2(util::get_from_file(10));

        day11::part1(util::get_test_from_file(11));
        day11::part1(util::get_from_file(11));
        day11::part2(util::get_from_file(11));
    }
    day12::part1(util::get_test_from_file(12));
    // day12::part1(util::get_from_file(12));
    // day12::part2(util::get_from_file(12));
}
