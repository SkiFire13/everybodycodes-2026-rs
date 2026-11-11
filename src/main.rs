mod client;

mod day1;
mod day2;
mod day3;

fn main() {
    let year = "2026";
    let day = "3";

    use day3 as day;

    println!("Part 1: {}", day::part1(&client::get_input(year, day, 1)));
    println!("Part 2: {}", day::part2(&client::get_input(year, day, 2)));
    println!("Part 3: {}", day::part3(&client::get_input(year, day, 3)));
}
