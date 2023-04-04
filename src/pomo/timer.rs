use std::time::{SystemTime,Duration};
use std::io::{stdout};
use crossterm::{execute,cursor,style::{SetForegroundColor,Color, SetAttribute, Attribute,Print},terminal::{self,ClearType}};
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers,poll};
pub struct Timer {
    duration: i32,
    min_left: i32,
    time_so_far: SystemTime,
    pub actual_studied: i32,
}

impl Timer {
    pub fn new(duration:i32) -> Timer {
        Timer{duration: duration, min_left: duration, time_so_far: SystemTime::now(),actual_studied: 0}
    }
    pub fn start(&mut self) {
        let mut stdout = stdout();
        let mut cur_sec: i32=0;
        terminal::enable_raw_mode().expect("Could not turn on Raw mode");
        execute!(stdout,cursor::Hide,cursor::MoveUp(1),terminal::Clear(ClearType::CurrentLine)
                 ,SetForegroundColor(Color::Red),SetAttribute(Attribute::Bold)
                 ,SetAttribute(Attribute::Italic),Print(self.min_left.to_string()+" minutes left \u{1F345}")).expect("could not hide cursor");
        while self.min_left >= 1 {
            if poll(Duration::from_millis(1)).unwrap() {
                if Timer::check_done() {
                    break;
                }
            } if let Ok(time) = self.time_so_far.elapsed() {
                if time.as_secs() as i32!=cur_sec&&time.as_secs()%60==0 {
                    self.min_left-=1;
                    execute!(stdout,terminal::Clear(ClearType::CurrentLine)).expect("could not clear");
                    execute!(stdout,Print("\r".to_owned()+&self.min_left.to_string()+" minutes left \u{1F345}")).expect("output issues");
                }
                cur_sec=time.as_secs() as i32;
            }
            //if self.time_so_far.elapsed()%60==0 {
            //    self.min_left-=1;
            //    println!("\r{} minutes left",self.min_left);
            //}
        }
        self.actual_studied=self.duration-self.min_left;
        execute!(stdout,terminal::Clear(ClearType::CurrentLine),Print("\rYou studied for ".to_owned()+&{self.actual_studied}.to_string()+" minutes ")).expect("could not print");
        terminal::disable_raw_mode().expect("Could not turn on Raw mode");
        execute!(stdout,cursor::Show).expect("could not hide cursor");
        println!("");
    }
    
    fn check_done() -> bool {
        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL, ..
            }) => true,
            _ => false
        }
    }
}
