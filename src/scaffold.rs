use std::fs;
use log::info;

const MOD_NAME: &str = r#"//INSERT MOD NAME//"#;
const PUB_MODULE: &str = r#"//INSERT PUB MOD HERE//"#;
const MATCH_DAY: &str = r#"//INSERT MATCH DAY HERE//"#;
const MATCH_NUMBER: &str = r#"//INSERT MATCH NUMBER HERE//"#;
const NUMBER: &str = r#"--INSERT--NUMBER--"#;

pub fn scaffold(day : u8) -> Option<()> {
    if day >24 {
        info!("There are not more than 24 day");
        return None;
    }

    if let Ok(r) = fs::exists(format!("src/days/day{day}"))  {
        if !r {
            insert_day_in_module(day)?;
            insert_day_module(day)?;
        } else { info!("Day {day} already exists"); }
    };
    Some(())
}


pub fn insert_day_in_module(day : u8) -> Option<()> {
    //Read mod.rs
    let m = fs::read_to_string("src/days/mod.rs").ok()?
    //Replace String Patterns

        .replace(MOD_NAME, &format!("day{day}::day{day}::Day{day},\n{MOD_NAME}"))
        .replace(PUB_MODULE, &format!("pub mod day{day};\n{PUB_MODULE}"))
        .replace(MATCH_DAY,&format!("\"Day{day}\" => {{ runner(day, &Day{day}) }}\n{MATCH_DAY}"))
        .replace(MATCH_NUMBER,&format!("{day} =>  {{Some(Box::new(Day{day}))}}\n{MATCH_NUMBER}"));
    //Write to mod.rs

    fs::write("src/days/mod.rs", m).ok()?;
    Some(())
}

pub fn insert_day_module(day : u8) -> Option<()> {
    //Read Day Templates
    let d = fs::read_to_string("src/resources/template/DayTemplate").ok()?.replace(NUMBER,&format!("{day}"));
    let dm = fs::read_to_string("src/resources/template/DaySubFolderTemplate").ok()?.replace(NUMBER,&format!("{day}"));



    //create Module Directory
    fs::create_dir_all(format!("src/days/day{day}")).ok()?;
    //
    if let Ok(r) = fs::exists(format!("src/days/day{day}/day{day}.rs")) {
        if !r {
            let _ = fs::write(format!("src/days/day{day}/day{day}.rs"), d);
        }
    }
    if let Ok(r) = fs::exists(format!("src/days/day{day}/mod.rs")) {
        if !r {
            let _ = fs::write(format!("src/days/day{day}/mod.rs"), dm);
        }
    }
    fs::write(format!("src/days/day{day}/testinputp1.txt"), "").ok()?;
    fs::write(format!("src/days/day{day}/testinputp2.txt"), "").ok()?;
    fs::write(format!("src/resources/inputDay{day}.txt"), "").ok()?;
    fs::write(format!("src/resources/testDataDay{day}.txt"), "").ok()?;

    Some(())
}