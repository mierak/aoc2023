use std::io::Write;

fn part(day: &usize, part: &i32) -> String {
    format!(
        r#"
    let start = std::time::Instant::now();
    match day{day}::part{part}(buf) {{
        Ok(res) => {{
            let elapsed = start.elapsed().as_micros() as f64 / 1000.0;
            println!("d{day:0>2}p{part} answer is: '{{}}' and took: {{:.5}}{{}}", res.as_str().green(), elapsed.to_string().yellow(), "ms".yellow());
        }}
        Err(err) => {{
            let elapsed = start.elapsed().as_micros() as f64 / 1000.0;
            println!("d{day:0>2}p{part} {{}} and took: {{:.5}}{{}}", "FAILED".bright_red(), elapsed.to_string().yellow(), "ms".yellow());
            println!("Failure message: {{}}", err);
        }}
    }};
"#
    )
}

fn input(day: &usize) -> String {
    format!(
        r#"    let buf = &mut String::new();
    std::fs::File::open("input/{day}").unwrap().read_to_string(buf).unwrap();
"#
    )
}

const IMPORTS: &[u8] = br#"use std::io::Read;
use colored::Colorize;
"#;

fn main() {
    let mut day_nums = std::fs::read_dir("src")
        .unwrap()
        .filter_map(|e| {
            let val = e.unwrap().file_name().to_string_lossy().to_string();
            if val.starts_with("day") {
                Some(
                    val.trim_end_matches(".rs")
                        .trim_start_matches("day")
                        .to_string()
                        .parse(),
                )
            } else {
                None
            }
        })
        .collect::<Result<Vec<usize>, _>>()
        .unwrap();
    day_nums.sort();

    let mut out = std::fs::File::create("src/lib.rs").unwrap();
    out.write_all(IMPORTS).unwrap();
    out.new_line();
    for day in day_nums.iter() {
        out.write_fmt(format_args!(r#"pub mod day{};"#, day)).unwrap();
        out.new_line();
    }
    out.new_line();

    out.write_all(b"pub fn run() {").unwrap();
    out.new_line();
    for day in day_nums.iter() {
        out.write_all(input(day).as_bytes()).unwrap();
        out.write_all(part(day, &1).as_bytes()).unwrap();
        out.write_all(part(day, &2).as_bytes()).unwrap();
    }
    out.write_all(b"}").unwrap();
}

trait FileExt {
    fn new_line(&mut self);
}

impl FileExt for std::fs::File {
    fn new_line(&mut self) {
        self.write_all(b"\n").unwrap();
    }
}
