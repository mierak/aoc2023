fn main() {
    let run_all = std::env::args().any(|v| v == "--all");
    aoc23::run(run_all);
}
