use crossterm::event::{self, poll, Event, KeyEventKind, KeyModifiers};
use std::time::Duration;

use crate::app::App;

pub fn udpate_0(app: &mut App) {
    if poll(Duration::from_millis(500)).unwrap() {
        if let Event::Key(key) = event::read().unwrap() {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    event::KeyCode::Char(x) => match x {
                        '!' => app.select_page(2),
                        '@' => app.select_page(2),
                        '#' => app.select_page(2),
                        '$' => app.select_page(2),
                        '%' => app.select_page(2),
                        '^' => app.select_page(2),
                        '&' => app.select_page(2),
                        _ => app.text_push(x),
                    },
                    event::KeyCode::Left => {
                        if key.modifiers.contains(KeyModifiers::SHIFT) {
                            app.prev_month();
                        } else {
                            app.prev_week();
                        }
                    }
                    event::KeyCode::Right => {
                        if key.modifiers.contains(KeyModifiers::SHIFT) {
                            app.next_month();
                        } else {
                            app.next_week();
                        }
                    },
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
                    event::KeyCode::Char(x) => app.text_push(x),
                    event::KeyCode::Backspace => app.text_pop(),
                    event::KeyCode::Delete => app.quit(),
                    event::KeyCode::Esc => app.select_page(0),
                    _ => {}
                }
            }
        }
    }
}
