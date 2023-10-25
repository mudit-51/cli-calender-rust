use chrono::prelude::*;

pub struct App {
    page: i32,
    should_quit: bool,
    page_text: String,
    days: Vec<Day>,
}

pub struct Day{
    date: String,
    today: bool,
}

impl App {
    pub fn new() -> App {
        let mut temp: Vec<Day> = Vec::new();

        let today = chrono::Local::now().date_naive().weekday();
        let mut days = today.week(Weekday::Mon).first_day();

        for _i in 0..7 {
            if days == today {
                temp.push(Day { date: days.to_string(), today: true });
            } else {
                temp.push(Day { date: days.to_string(), today: false });
            }
            days = days.succ_opt().unwrap();
        }
        App {
            page: 0,
            should_quit: false,
            page_text: String::new(),
            days: temp,
        }
    }
    pub fn get_page_text(&self) -> String {
        self.page_text.clone()
    }
    pub fn text_push(&mut self, c: char) {
        self.page_text.push(c)
    }
    pub fn text_pop(&mut self) {
        match self.page_text.pop() {
            Some(_) => {}
            None => {}
        };
    }
    pub fn get_status(&self) -> bool {
        self.should_quit
    }
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
    pub fn select_page(&mut self, i: i32) {
        self.page = i;
        self.page_text.clear();
    }
    pub fn get_page(&self) -> i32 {
        self.page
    }
    pub fn get_days(&self) -> &Vec<Day>{
        &self.days
    }
}

impl Day{
    pub fn get_date_string(&self) -> &String{
        &self.date
    }
    pub fn is_today(&self) -> bool{
        self.today
    }
}
