use chrono::{prelude::*, Days, Months};

pub struct App {
    page: i32,
    should_quit: bool,
    page_text: String,
    days: Vec<Day>,
    scroll_state: (u16, u16),
}

pub struct Day {
    date: NaiveDate,
    today: bool,
    tasks: Option<Vec<String>>,
}


impl App {
    pub fn new() -> App {
        let mut temp: Vec<Day> = Vec::new();

        let today = chrono::Local::now().date_naive();
        let mut day = today.week(Weekday::Mon).first_day();

        for _i in 0..7 {
            temp.push(Day {
                date: day,
                today: day==today,
                tasks: None,
            });
            day = day.succ_opt().unwrap();
        }
        App {
            page: 0,
            should_quit: false,
            page_text: String::new(),
            days: temp,
            scroll_state: (0, 0),
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
    pub fn get_days(&self) -> &Vec<Day> {
        &self.days
    }
    pub fn get_scroll(&self) -> (u16, u16) {
        self.scroll_state
    }
    pub fn set_scroll_vertical(&mut self, offset: i16) {
        let temp = self.scroll_state.0;
        match temp.checked_add_signed(offset) {
            Some(x) => self.scroll_state = (x, self.scroll_state.1),
            None => self.scroll_state = (temp, self.scroll_state.1),
        }
    }
    pub fn set_scroll_horizontal(&mut self, offset: i16) {
        let temp = self.scroll_state.1;
        match temp.checked_add_signed(offset) {
            Some(x) => self.scroll_state = (self.scroll_state.0, x),
            None => self.scroll_state = (self.scroll_state.0, temp),
        }
    }
    pub fn next_week(&mut self) {
        let mut temp: Vec<Day> = Vec::new();
        for i in &self.days {
            let next_day = i.date.checked_add_days(Days::new(7)).unwrap();
            temp.push(Day {
                date: next_day,
                today: next_day == chrono::Local::now().date_naive(),
                tasks: None,
            });
        }
        self.days = temp;
    }
    pub fn next_month(&mut self) {
        let mut temp: Vec<Day> = Vec::new();
        for i in &self.days {
            let next_day = i.date.checked_add_months(Months::new(1)).unwrap();
            temp.push(Day {
                date: next_day,
                today: next_day == chrono::Local::now().date_naive(),
                tasks: None,
            });
        }
        self.days = temp;
    }
    pub fn prev_week(&mut self) {
        let mut temp: Vec<Day> = Vec::new();
        for i in &self.days {
            let next_day = i.date.checked_sub_days(Days::new(7)).unwrap();
            temp.push(Day {
                date: next_day,
                today: next_day == chrono::Local::now().date_naive(),
                tasks: None,
            });
        }
        self.days = temp;
    }
    pub fn prev_month(&mut self) {
        let mut temp: Vec<Day> = Vec::new();
        for i in &self.days {
            let next_day = i.date.checked_sub_months(Months::new(1)).unwrap();
            temp.push(Day {
                date: next_day,
                today: next_day == chrono::Local::now().date_naive(),
                tasks: None,
            });
        }
        self.days = temp;
    }
}

impl Day {
    pub fn get_date(&self) -> &NaiveDate {
        &self.date
    }
    pub fn is_today(&self) -> bool {
        self.today
    }
    pub fn get_tasks(&self) -> &Option<Vec<String>>{
        &self.tasks
    }
}
