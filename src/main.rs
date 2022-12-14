pub mod file;
pub mod forest;

pub mod iters;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

type ProblemFn = Box<dyn Fn(Vec<String>) -> String>;

fn string_wrap<F: std::fmt::Debug>(f: impl Fn(Vec<String>) -> F + 'static) -> ProblemFn {
    Box::new(move |v| format!("{:?}", f(v)))
}

fn problems() -> Vec<Vec<ProblemFn>> {
    vec![
        vec![string_wrap(day1::part1), string_wrap(day1::part2)],
        vec![string_wrap(day2::part1), string_wrap(day2::part2)],
        vec![string_wrap(day3::part1), string_wrap(day3::part2)],
        vec![string_wrap(day4::part1), string_wrap(day4::part2)],
        vec![string_wrap(day5::part1), string_wrap(day5::part2)],
        vec![string_wrap(day6::part1), string_wrap(day6::part2)],
        vec![string_wrap(day7::part1), string_wrap(day7::part2)],
        vec![string_wrap(day8::part1), string_wrap(day8::part2)],
        vec![string_wrap(day9::part1), string_wrap(day9::part2)],
        vec![string_wrap(day10::part1), string_wrap(day10::part2)],
        vec![string_wrap(day11::part1), string_wrap(day11::part2)],
        vec![string_wrap(day12::part1), string_wrap(day12::part2)],
        vec![string_wrap(day13::part1), string_wrap(day13::part2)],
        vec![string_wrap(day14::part1), string_wrap(day14::part2)],
        vec![string_wrap(day15::part1), string_wrap(day15::part2)],
        vec![string_wrap(day16::part1), string_wrap(day16::part2)],
    ]
}

fn main() {
    let mut args = std::env::args();

    args.next();

    let day: usize = args
        .next()
        .expect("Expect day argument")
        .parse()
        .expect("Expect day argument");
    let part: usize = args
        .next()
        .expect("Expect part argument")
        .parse()
        .expect("Expect part argument");
    let file = args.next().expect("Expect file argument");

    let p = problems();
    let func = &p[day - 1][part - 1];

    let timer = std::time::Instant::now();

    match file::read_lines(&file) {
        Ok(lines) => println!("{}", func(lines)),
        Err(e) => println!("Error reading file ({}) {}", file, e),
    }

    println!("{:.2?}", timer.elapsed());
}
