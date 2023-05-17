use crossterm::{execute,cursor,style::Print};
use crossterm::terminal::{self,ClearType};
use std::io::{stdout};
use std::cmp::max;

//finds max element in vector
fn max_elem(vec: &Vec<i32>) -> i32{
    let mut max_num: i32 = 0;
    for i in vec {
        max_num=max(*i,max_num);
    }
    max_num
}

//displays chart
pub fn display_chart(days: &Vec<i32>) {
    let day_names:Vec<char> = vec!['M','T','W','R','F','S','U'];
    let factor = (max_elem(days)+24)/25;
    let amt_right;

    //finds amt needed to shift the bars right
    if 25*factor<=0 {
        amt_right=1; 
    } else {
        amt_right = ((25*factor).ilog10()+2) as u16;
    }
    
    //Sets up top line in chart
    terminal::enable_raw_mode().expect("Could not turn on Raw mode");
    execute!(stdout(), terminal::Clear(ClearType::All), cursor::MoveTo(0,0),
        Print("+------------------------------------------------------"),cursor::MoveTo(0,0)).expect("Output issues");

    //Sets up numbering for chart
    for i in (0..26).rev() {
        //finds out how many digits y num is
        let amt_left;
        if i*factor<=0 {
            amt_left=1; 
        } else {
            amt_left = ((i*factor).ilog10()+1) as u16;
        }

        execute!(stdout(),cursor::MoveDown(1),Print(i*factor),cursor::MoveLeft(amt_left)).expect("output issues");
    }

    //Prints out the bars
    let mut ctr=0;
    for i in days {

        if factor==0 || *i/factor==0 {
            execute!(stdout(),cursor::MoveTo(amt_right+7*ctr,0),cursor::MoveDown(26)
                     ,Print("+-".to_owned()+&day_names[ctr as usize].to_string()+"-+")).expect("output issues");
            ctr+=1;
            continue;
        }

        execute!(stdout(),cursor::MoveTo(amt_right+7*ctr,0)).expect("output issues");
        for _j in 0..26-*i/factor {
            execute!(stdout(),cursor::MoveDown(1)).expect("output issues");
        }

        execute!(stdout(),Print("+---+"),cursor::MoveLeft(5)).expect("output issues");
        for _j in 0..*i/factor-1 {
            execute!(stdout(),cursor::MoveDown(1),Print("|   |"),cursor::MoveLeft(5)).expect("output issues");
        }

        execute!(stdout(),cursor::MoveDown(1),Print("+-".to_owned()+&day_names[ctr as usize].to_string()+"-+")).expect("output issues");
        ctr+=1;
    }

    terminal::disable_raw_mode().expect("Could not turn off");
    print!("\n");
}
