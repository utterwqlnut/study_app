use crate::pomo::timer::Timer;
use std::env;
use crate::graph::bar;
use std::fs::File;
use std::io::{self,prelude::*,BufReader};
use chrono::Datelike;
pub mod graph;
pub mod pomo;
fn main() -> io::Result<()>{
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    
    //A lot of reading in of files
    let file = File::open("days.txt")?;
    let reader = BufReader::new(&file);
    let mut days: Vec<i32> = Vec::new();
    for line in reader.lines() {
        let value:i32 = line?.trim().parse().expect("Please enter a number");
        days.push(value);
    }
    let mut file = File::create("days.txt")?;

    let last_seen_file = File::open("last_seen.txt")?;
    let mut last_seen_reader = BufReader::new(&last_seen_file);
    let mut last_val = String::new();

    last_seen_reader.read_line(&mut last_val).expect("Issue reading");
    let last_val: usize = last_val.trim().parse().expect("Please enter a number");
    let day_val: usize;
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
    //resetting days to 0 if needed
    if day_val != last_val {
        if day_val < last_val {
            for i in 0..7 {
                days[i]=0;
            }
        } else {
            for i in last_val..=day_val { 
                days[i]=0;
            }
        }   
    }

    //updates to last_seen.txt
    let mut last_seen_file = File::create("last_seen.txt")?;
    let mut string = String::new();
    last_seen_file.write_all(&day_val.to_string().as_bytes())?;

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
        string+=&(day.to_string()+"\n");
    }

    file.write_all(&string.as_bytes())?;
    Ok(())
}
