use std::fs::{self, File};

use chrono::{prelude::*, Days};
use serde_json::Value;

//Implements the "model" part of the application

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

        //Gets present date. Stores what date it was on Monday of the present week into "day" variable.
        let today = chrono::Local::now().date_naive();
        let mut day = today.week(Weekday::Mon).first_day();

        //Checks if json file with app data exists. Creates one if it doesn't. Panics if it can't create it. 
        if let Err(_) = File::open("data.json") {
            if let Err(_) = fs::write("data.json", "{}") {
                panic!("Error while creating data.json file")
            }
        }
        
        //Reads json file with task data for each corresponding date and then converts it into a hashmap.
        let json: serde_json::Value =
            serde_json::from_reader(File::open("data.json").unwrap()).unwrap();

        let x = json.as_object().unwrap();

        //Since 7 days are displayed on the main page, it iterates 7 times, beginning from Monday of present week
        //to Sunday. For each such day, it checks if tasks for that day exist, and then stores that value inside
        //an option of Day type. 
        //This Day struct is then pushed to a vector, "temp", which is then added to App struct. App struct is then 
        //returned
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

    //Returns the user text input on page. Mainly relevant for add task page. 
    pub fn get_page_text(&self) -> String {
        self.page_text.clone()
    }
    
    //Sets user text on page. Mainly relevant for add task page. 
    pub fn set_page_text(&mut self, text: String) {
        self.page_text = text;
    }
    
    //Pushes new charecter to existing text on page. Used to give the feeling that user is actually inputting text.
    //Mainly relevant for add task page. 
    pub fn text_push(&mut self, c: char) {
        self.page_text.push(c)
    }
    
    //Pops charecter from existing text on page. Used to give the feeling that user is actually backspacing.
    //Mainly relevant for add task page.
    pub fn text_pop(&mut self) {
        match self.page_text.pop() {
            Some(_) => {}
            None => {}
        };
    }

    //Returns if application should quit or not. 
    pub fn get_status(&self) -> bool {
        self.should_quit
    }

    //Function is executed when application quits. It saves all the changes the user made and writes them to a new
    //json file that overwrites the old file. Returns error if it can't write the new file. 
    pub fn quit(&mut self) {
        self.should_quit = true;
        let new_json = serde_json::to_string_pretty(&self.task_json).unwrap();
        fs::write("data.json", new_json).expect("Error while trying to save file");
    }

    //Sets which to display. Also takes target_day as parameter so that when we go from main page -> add task page,
    //we know what day to add the task in. 
    //Also clears the user text input.
    pub fn select_page(&mut self, i: i32, target_day: usize) {
        self.page = i;
        self.day_task_add = target_day;
        self.page_text.clear();
    }

    //Returns the number of the page currently being displayed. 
    pub fn get_page(&self) -> i32 {
        self.page
    }

    //Returns a vector that contains all the days that are currently being displayed on the user's screen. 
    pub fn get_days(&self) -> &Vec<Day> {
        &self.days
    }

    //Returns current scroll state
    pub fn get_scroll(&self) -> (u16, u16) {
        self.scroll_state
    }

    //Function to set the vertical scroll. 
    pub fn set_scroll_vertical(&mut self, offset: i16) {
        let temp = self.scroll_state.0;
        match temp.checked_add_signed(offset) {
            Some(x) => self.scroll_state = (x, self.scroll_state.1),
            None => self.scroll_state = (temp, self.scroll_state.1),
        }
    }
    
    //Function to set the horizontal scroll. 
    pub fn set_scroll_horizontal(&mut self, offset: i16) {
        let temp = self.scroll_state.1;
        match temp.checked_add_signed(offset) {
            Some(x) => self.scroll_state = (self.scroll_state.0, x),
            None => self.scroll_state = (self.scroll_state.0, temp),
        }
    }

    //Function to change the days that are currently being displayed on the user's screen. 
    //Can only scroll to next/previous week or next/previous month. 
    //Any other value of "time" as input to this function will have no effect.  
    //Behaves very similarly to the code in App::new() function. 
    pub fn next(&mut self, time: i32) {
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

    //With the help of select_page() function, it is already known to what index of Days field the new task needs
    //to be inserted.
    //If user text input is of length 0, nothing happens and function simply returns early.
    //The day to which new task must be added is stored in target_day.
    //The contents of json file are stored in json map as a mutable hashmap. 
    //If date in hashmap already exists, then new task is pushed to the vector containing existing tasks.
    //Else a new key for that date is created. Then a new vector containing the new task is set as the value of that key.
    //After addition, add task page is set back to main page. User text input is also cleared. 
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

    //Clears all the task from a Day. 
    //Behaves similarly to add_task() function.
    //Instead of adding task, the value stored in the key-value pair is simply destroyed.  
    pub fn clear_tasks(&mut self, day: usize) {
        self.day_task_add = day;

        let target_day = self
            .days
            .get(self.day_task_add)
            .expect("Fatal error occured");

        let json_map = self.task_json.as_object_mut().unwrap();

        json_map.remove(&target_day.date.to_string());
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
