use std::fs;
use tracing::warn;
use log::info;

use super::days::{
    //INSERT MOD NAME//
};

//INSERT PUB MOD HERE//
pub mod benchmark;

trait Day {

    fn run_part1(&self,input : &str) -> Option<u32>;
    fn run_part2(&self,input : &str) -> Option<u32>;

}



pub fn run_day(day : &str ) {
    match day {
        //INSERT MATCH DAY HERE//
        &_ => {}
    }
}

pub fn get_day(day : u8) -> Option<Box<dyn Day>> {
    match day {
        //INSERT MATCH NUMBER HERE//
        _ => None
    }
}



pub fn runner(day_input: &str, d : &impl Day ) {
    let input = fs::read_to_string(format!("src/resources/input{}.txt",day_input));
    if let Ok(input_str) = input {

        let res_part1 = d.run_part1(&input_str);
        info!("Result {day_input} part1 : '{res_part1:?}'"  );
        let res_part2 = d.run_part2(&input_str);
        info!("Result {day_input} part2 : '{res_part2:?}'"  )

    }else {
        warn!("Input file in 'src/resources/input{}.txt'",day_input)
    }


}
