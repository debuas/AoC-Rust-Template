use std::fs;
use std::fs::File;
use std::io::{Write};
use std::time::Instant;
use itertools::Itertools;
use log::{debug, info};
use regex::Regex;
use crate::days::Day;

use crate::days::get_day;

#[derive(Clone,Default,Debug)]
pub struct BenchmarkData {
    part1 : Option<std::time::Duration>,
    part2 : Option<std::time::Duration>
}

const TABLE_HEADER: &str = "| Days | Test Part 1 | Test Part 2 | Part 1 | Part 2 | ";
const TABLE_SPLIT: &str = "| ---- | ----------- | ----------- | ------ | ------ | ";
const PLACE_HOLDER : &str=  " - ";

const REPLACE_COMMENT : &str= "<!--- BENCHMARK --->";
const BENCHMARK_END_COMMENT : &str= "<!--- BENCHMARK END --->";

#[allow(dead_code)]
pub enum BenchMarkSetting<'a>{
    Day(u8),
    Days(&'a [u8]),
    DayRange{min: u8 , max : u8},
    All
}

impl BenchMarkSetting<'_> {

    pub fn run_benchmark(&self) {
        let days : Vec<u8> = match self {
            BenchMarkSetting::Day(d) => {vec![*d]}
            BenchMarkSetting::Days(ref d) => {Vec::from(*d)}
            BenchMarkSetting::DayRange { ref min, ref max } => {(*min..*max).collect()}
            BenchMarkSetting::All => {(1..25).collect() }
        };

        let res =days.iter()
            .map(|d|(d,benchmark_day(*d)))
            .map(|(d,b)|{
                if let Some(bench) =b{
                    match bench {
                        (Some(a),Some(b)) => (d,generate_markdown_benchmark_row_all(d,&a,&b)),
                        (None,Some(b)) => (d,generate_markdown_benchmark_row_real_only(d,&b)),
                        (Some(a),None) => (d,generate_markdown_benchmark_row_test_only(d,&a)),
                        _ => (d,generate_markdown_benchmark_empty_row(d))
                    }
                } else {(d,generate_markdown_benchmark_empty_row(d))}

        }).collect_vec();

        let file = File::create("./Benchmark.md");

        if let Ok(mut f) = file {
            let _ = f.write(TABLE_HEADER.as_bytes());
            let _ = f.write("\n".as_bytes());
            let _ = f.write(TABLE_SPLIT.as_bytes());
            let _ = f.write("\n".as_bytes());
            res.iter().for_each(|(_i,b)|{
                let _ = f.write(b.as_bytes());
                let _ = f.write("\n".as_bytes());
            })


        }

        debug!("Benchmark generated Rows : {:?}", res);
    }

}




pub fn get_code_path(day : &u8) -> String {
    format!("./src/days/day{day}/day{day}.rs")
}

pub fn get_test_data(day : u8) -> Option<String> {
    fs::read_to_string(format!("src/resources/testDataDay{}.txt",day)).ok()
}
pub fn get_day_input_data(day : u8) -> Option<String> {
    fs::read_to_string(format!("src/resources/inputDay{}.txt",day)).ok()
}


pub fn bench_fn<F, I, O>(func: F, input: I) -> (Option<O>, std::time::Duration)
where
    F: FnOnce(I) -> Option<O>,
{
    let start_time = Instant::now(); // start time
    let result = func(input); // execute function
    let elapsed_time = start_time.elapsed(); // measured time

    (result,elapsed_time)
}

//Returns Benchmark on Mock Input and Real Input Data
pub fn benchmark_day(day_index: u8) -> Option<(Option<BenchmarkData>,Option<BenchmarkData>)> {

    let day = get_day(day_index)?;
    info!("Currently Running Benchmark for day {}", day_index);
    let p1 = |x : String| day.run_part1(&x);
    let p2 = | x : String| day.run_part2(&x);


    let testpart1 = {
        if let Some(input) = get_test_data(day_index) {
            let res = bench_fn(p1, input);
            match res {
                (Some(_),time) => Some(time),
                _ => None
            }
        } else { None }

    };
    let testpart2 = {
        if let Some(input) = get_test_data(day_index) {
            let res = bench_fn(p2, input);
            match res {
                (Some(_),time) => Some(time),
                _ => None
            }
        } else { None }

    };
    let part1 = {
        if let Some(input) = get_day_input_data(day_index) {
            let res = bench_fn(p1, input);
            match res {
                (Some(_),time) => Some(time),
                _ => None
            }
        } else { None }

    };
    let part2 = {
        if let Some(input) = get_day_input_data(day_index) {
            let res = bench_fn(p2, input);
            match res {
                (Some(_),time) => Some(time),
                _ => None
            }
        } else { None }

    };

    let res = Some((
            Some(BenchmarkData{part1:testpart1,part2:testpart2}),
            Some(BenchmarkData{part1,part2})
        ));
    info!("Day {day_index} Benchmark Results {:?}", res);
    res
}

fn generate_markdown_benchmark_row_all(day : &u8, test_benchmark : &BenchmarkData, result_benchmark : &BenchmarkData) -> String {
    let tp1 = if let Some(d) = test_benchmark.part1 {format!("{d:?}")} else { " - ".to_string() };
    let tp2 = if let Some(d) = test_benchmark.part2 {format!("{d:?}")} else { " - ".to_string() };
    let p1 = if let Some(d) = result_benchmark.part1 {format!("{d:?}")} else { " - ".to_string() };
    let p2 = if let Some(d) = result_benchmark.part2 {format!("{d:?}")} else { " - ".to_string() };
    let mdr = format!("| [day{day}]({}) | {tp1} | {tp2} | {p1} | {p2} |",get_code_path(day));
    debug!("generated : {mdr}");

    mdr
}

fn generate_markdown_benchmark_empty_row(day : &u8) -> String{

    let mdr = format!("| day{day} | {PLACE_HOLDER} | {PLACE_HOLDER} | {PLACE_HOLDER} | {PLACE_HOLDER} |",);
    debug!("generated : {mdr}");
    mdr
}

fn generate_markdown_benchmark_row_test_only(day : &u8, test_benchmark : &BenchmarkData,) -> String {
    let tp1 = if let Some(d) = test_benchmark.part1 {format!("{d:?}")} else { " - ".to_string() };
    let tp2 = if let Some(d) = test_benchmark.part2 {format!("{d:?}")} else { " - ".to_string() };
    let mdr = format!("| [day{day}]({}) | {tp1} | {tp2} | {PLACE_HOLDER} | {PLACE_HOLDER} |",get_code_path(day));
    debug!("generated : {mdr}");

    mdr
}

fn generate_markdown_benchmark_row_real_only(day : &u8, result_benchmark : &BenchmarkData) -> String {
    let p1 = if let Some(d) = result_benchmark.part1 {format!("{d:?}")} else { " - ".to_string() };
    let p2 = if let Some(d) = result_benchmark.part2 {format!("{d:?}")} else { " - ".to_string() };
    let mdr = format!("| [day{day}]({}) | {PLACE_HOLDER} | {PLACE_HOLDER} | {p1} | {p2} |",get_code_path(day));
    debug!("generated : {mdr}");

    mdr
}

pub fn update_readme() -> Option<()> {

    let bm = fs::read_to_string("./Benchmark.md").ok()?;
    let re = Regex::new(&format!(
        r"(?s){}.*?{}",
        regex::escape(REPLACE_COMMENT),
        regex::escape(BENCHMARK_END_COMMENT)
    )).unwrap();
    let md = &fs::read_to_string("./README.md").ok()?;
    let modified = re.replace(&md , &format!("{REPLACE_COMMENT}\n{bm}\n{BENCHMARK_END_COMMENT}"));

    debug!("{}", modified);
    File::create("./README.md").ok()?.write(modified.as_bytes()).ok()?;

    None
}
