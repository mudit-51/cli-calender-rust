use std::fs::{self, File};

use chrono::{prelude::*, Days};
use serde_json::Value;

pub struct App {
    page: i32,
    should_quit: bool,
    page_text: String,
    days: Vec<Day>,
    scroll_state: (u16, u16),
    task_json: serde_json::Value,
    day_task_add: usize,
}

pub struct Day {
    date: NaiveDate,
    today: bool,
    tasks: Option<Value>,
}

impl App {
    pub fn new() -> App {
        let mut temp: Vec<Day> = Vec::new();

        let today = chrono::Local::now().date_naive();
        let mut day = today.week(Weekday::Mon).first_day();

        let json: serde_json::Value =
            serde_json::from_reader(File::open("data.json").unwrap()).unwrap();

        let x = json.as_object().unwrap();

        for _i in 0..7 {
            let tasks = match x.get(&day.to_string()) {
                Some(x) => Some(x.clone().to_owned()),
                None => None,
            };
            temp.push(Day {
                date: day,
                today: day == today,
                tasks,
            });
            day = day.succ_opt().unwrap();
        }
        App {
            page: 0,
            should_quit: false,
            page_text: String::new(),
            days: temp,
            scroll_state: (0, 0),
            task_json: json,
            day_task_add: 0,
        }
    }
    pub fn get_page_text(&self) -> String {
        self.page_text.clone()
    }
    pub fn set_page_text(&mut self, text: String) {
        self.page_text = text;
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
        let new_json = serde_json::to_string_pretty(&self.task_json).unwrap();
        fs::write("data.json", new_json).expect("Error while trying to save file");
    }
    pub fn select_page(&mut self, i: i32, target_day: usize) {
        self.page = i;
        self.day_task_add = target_day;
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
    pub fn next(&mut self, time: i32){
        let obj = self.task_json.as_object().unwrap();

        let mut temp: Vec<Day> = Vec::new();
        for i in &self.days {
            let next_day = match time {
                7 => i.date.checked_add_days(Days::new(7)).unwrap(),
                28 => i.date.checked_add_days(Days::new(28)).unwrap(),
                -7 => i.date.checked_sub_days(Days::new(7)).unwrap(),
                -28 => i.date.checked_sub_days(Days::new(28)).unwrap(),
                _ => i.date,
            };
            let tasks = match obj.get(&next_day.to_string()) {
                Some(x) => Some(x.clone().to_owned()),
                None => None,
            };
            temp.push(Day {
                date: next_day,
                today: next_day == chrono::Local::now().date_naive(),
                tasks,
            });
        }
        self.days = temp;
    }
    pub fn add_task(&mut self) {
        if self.page_text.len() == 0 {
            self.page = 0;
            return;
        };

        let target_day = self
            .days
            .get(self.day_task_add)
            .expect("Fatal error occured");

        let json_map = self.task_json.as_object_mut().unwrap();

        if let Some(d) = json_map.get_mut(&target_day.date.to_string()) {
            d.as_array_mut()
                .unwrap()
                .push(Value::String(String::from(&self.page_text)));
        } else {
            json_map.insert(
                target_day.date.to_string(),
                Value::Array(vec![Value::String(String::from(&self.page_text))]),
            );
        }
        self.page_text.clear();
        self.page = 0;
        self.next(0);
    }
}

impl Day {
    pub fn get_date(&self) -> &NaiveDate {
        &self.date
    }
    pub fn is_today(&self) -> bool {
        self.today
    }
    pub fn get_tasks(&self) -> &Option<Value> {
        &self.tasks
    }
}
