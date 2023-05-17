use crate::pomo::timer::Timer;
use std::env;
use crate::graph::bar;
use std::fs::File;
use std::io::{self,prelude::*,BufReader};
use chrono::Datelike;
use chrono::Utc;
pub mod graph;
pub mod pomo;
fn main() -> io::Result<()>{
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    
    //Reads in days values
    let file = File::open("days.txt")?;
    let reader = BufReader::new(&file);
    let mut days: Vec<i32> = Vec::new();
    for line in reader.lines() {
        let value:i32 = line?.trim().parse().expect("Please enter a number");
        days.push(value);
    }
    
    //reads in last_seen file 
    let mut file = File::create("days.txt")?;

    let last_seen_file = File::open("last_seen.txt")?;
    let mut last_seen_reader = BufReader::new(&last_seen_file);
    let mut last_val = String::new();
    let mut last_days_from_ce = String::new();
    let day_val: usize;

    last_seen_reader.read_line(&mut last_val).expect("Issue reading");
    let last_day_val: usize = last_val.trim().parse().expect("Issue parsing");
    last_seen_reader.read_line(&mut last_days_from_ce).expect("Issue reading");
    let last_days_from_ce_val: i32 = last_days_from_ce.trim().parse().expect("issue parsing");

    //converting weekday to number
    let weekday = chrono::offset::Local::now().date_naive().weekday();
    match weekday {
        chrono::Weekday::Mon => day_val=0,
        chrono::Weekday::Tue => day_val=1,
        chrono::Weekday::Wed => day_val=2,
        chrono::Weekday::Thu => day_val=3,
        chrono::Weekday::Fri => day_val=4,
        chrono::Weekday::Sat => day_val=5,
        chrono::Weekday::Sun => day_val=6,
    }

    let num_days_from_ce_val: i32 = Utc::now().num_days_from_ce();
    
    //resetting days to 0 if needed
    //Checks if its been a week since program was last run
    if num_days_from_ce_val - last_days_from_ce_val >= 7 {
        for i in 0..7 {
            days[i]=0;
        }
    }
    else if day_val != last_day_val {
        if day_val < last_day_val {
            for i in 0..7 {
                days[i]=0;
            }
        } else {
            for i in last_day_val..=day_val { 
                days[i]=0;
            }
        }   
    }

    //updates to last_seen.txt
    let mut last_seen_file = File::create("last_seen.txt")?;
    let mut result1 = String::new();
    let mut result2 = String::new();
    result2+=&day_val.to_string();
    result2+="\n";
    result2+=&num_days_from_ce_val.to_string();

    last_seen_file.write_all(result2.as_bytes())?;

    //handles each query
    if query.eq(&String::from("reset")) {
        for i in 0..7 {
            days[i] = 0;
        }
    }
    else if query.eq(&String::from("pomo")) {
        let mut duration = String::new();
        println!("Enter how how long the timer should last");
        io::stdin().read_line(&mut duration).expect("Failed to read line");
        let duration: i32 = duration.trim().parse().expect("Please Type a number");
        let mut timer = Timer::new(duration);
        timer.start(); 

        days[day_val]+=timer.actual_studied;
    } else if query.eq(&String::from("graph")) {
        bar::display_chart(&days);
        
    } else {
        println!("{} is not a command.",query);
        return Ok(());
    }
    //updates days.txt
    for day in &days {
        result1+=&(day.to_string()+"\n");
    }

    file.write_all(&result1.as_bytes())?;
    Ok(())
}
