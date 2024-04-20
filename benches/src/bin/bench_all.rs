use std::time::Instant;

const RUNS: usize = 100;
fn main() {
    println!("Running benchmark for all days {} times", RUNS);

    let jobs = jobs();

    let times = jobs
        .iter()
        .map(|(name, job)| {
            let time = (0..RUNS)
                .map(|_| {
                    let start = Instant::now();
                    job();
                    start.elapsed()
                })
                .min()
                .unwrap();
            (name, time)
        })
        .collect::<Vec<_>>();

    println!("\nResults:");
    times.iter().for_each(|(name, time)| {
        println!("{}: {:?}", name, time);
    });
}

fn jobs() -> Vec<(&'static str, fn())> {
    vec![
        ("day1", day1::main),
        ("day2", day2::main),
        ("day3", day3::main),
        ("day4", day4::main),
        ("day5", day5::main),
        ("day6", day6::main),
        // ("day7", day7::main),
        // ("day8", day8::main),
        // ("day9", day9::main),
        // ("day10", day10::main),
        // ("day11", day11::main),
        // ("day12", day12::main),
        // ("day13", day13::main),
        // ("day14", day14::main),
        // ("day15", day15::main),
        // ("day16", day16::main),
        // ("day17", day17::main),
        // ("day18", day18::main),
        // ("day19", day19::main),
        // ("day20", day20::main),
        // ("day21", day21::main),
        // ("day22", day22::main),
        // ("day23", day23::main),
        // ("day24", day24::main),
        // ("day25", day25::main),
    ]
}
