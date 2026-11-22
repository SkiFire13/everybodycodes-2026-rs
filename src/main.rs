use quest_helper::{init_everybodycodes_client, setup_info};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;

mod day20;

fn main() {
    quest_helper::run(
        init_everybodycodes_client!(),
        "2026",
        setup_info!("./day_template.rs"),
    );
}
