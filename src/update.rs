use crossterm::event::{self, poll, Event, KeyEventKind, KeyModifiers};
use std::time::Duration;

use crate::app::App;

pub fn udpate_0(app: &mut App) {
    if poll(Duration::from_millis(500)).unwrap() {
        if let Event::Key(key) = event::read().unwrap() {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    event::KeyCode::Char(x) => match x {
                        '1' => app.select_page(1,0),
                        '2' => app.select_page(1,1),
                        '3' => app.select_page(1,2),
                        '4' => app.select_page(1,3),
                        '5' => app.select_page(1,4),
                        '6' => app.select_page(1,5),
                        '7' => app.select_page(1,6),
                        _ => {},
                    },
                    event::KeyCode::Left => {
                        if key.modifiers.contains(KeyModifiers::SHIFT) {
                            app.next(-30);
                        } else {
                            app.next(-7);
                        }
                    }
                    event::KeyCode::Right => {
                        if key.modifiers.contains(KeyModifiers::SHIFT) {
                            app.next(30);
                        } else {
                            app.next(7);
                        }
                    }
                    event::KeyCode::Up => app.set_scroll_vertical(-3),
                    event::KeyCode::Down => app.set_scroll_vertical(3),
                    event::KeyCode::Backspace => app.text_pop(),
                    event::KeyCode::Delete => app.quit(),
                    _ => {}
                }
            }
        }
    }
}
pub fn update_1(app: &mut App) {
    if poll(Duration::from_millis(500)).unwrap() {
        if let Event::Key(key) = event::read().unwrap() {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    event::KeyCode::Enter => app.add_task(),
                    event::KeyCode::Char(x) => app.text_push(x),
                    event::KeyCode::Backspace => app.text_pop(),
                    event::KeyCode::Delete => app.quit(),
                    event::KeyCode::Esc => app.select_page(0,0),
                    _ => {}
                }
            }
        }
    }
}
