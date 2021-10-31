use console::Term;
use core::time;
use indicatif::{ProgressBar, ProgressStyle};
use notify_rust::Notification;
use std::thread;

use crate::config::TomatoConfig;

pub struct Session {
    state: State,
    work_count: u64,
    config: TomatoConfig,
}

#[derive(Debug, PartialEq, Eq)]
pub enum State {
    None,
    Working,
    ShortBreak,
    LongBreak,
}

impl Session {
    pub fn new(config: TomatoConfig) -> Self {
        Session {
            work_count: 0,
            state: State::None,
            config,
        }
    }

    fn next_state(&self) -> State {
        match self.state {
            State::None => State::Working,
            State::Working => {
                if self.work_count >= 4 {
                    State::LongBreak
                } else {
                    State::ShortBreak
                }
            }
            State::LongBreak => State::Working,
            State::ShortBreak => State::Working,
        }
    }

    fn next(&mut self) {
        self.state = self.next_state();
        if self.state == State::LongBreak {
            self.work_count = 0;
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.state {
                State::None => {}
                State::Working => {
                    notify(&format!("🍅 Start work time #{}", self.work_count + 1));
                    process_state(self.config.work_time, "Working...");
                    self.work_count += 1;
                }
                State::ShortBreak => {
                    notify("🍅 Start short break time");
                    process_state(self.config.short_break_time, "Short Breaking...")
                }
                State::LongBreak => {
                    notify("🍅 Start long break time");
                    process_state(self.config.long_break_time, "Long Breaking...");
                }
            }
            self.next();
        }
    }
}

fn notify(message: &str) {
    Notification::new()
        .body(message)
        .sound_name("Ping")
        .show()
        .unwrap();
}

fn process_state(duration: u64, state_msg: &str) {
    let term = Term::stdout();
    let duration_second = duration * 60;

    term.clear_screen().unwrap();
    term.write_line(&format!("Tomato: {}", state_msg)).unwrap();

    let progress = ProgressBar::new(duration_second);
    progress.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}]"),
    );
    progress.inc(0);

    for _i in 0..duration_second {
        thread::sleep(time::Duration::from_secs(1));
        progress.inc(1);
    }

    progress.finish();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_state_none_should_be_working() {
        let config = TomatoConfig::new(25, 5, 15);
        let session = Session {
            work_count: 0,
            state: State::None,
            config,
        };
        let next = session.next_state();
        assert_eq!(next, State::Working);
    }

    #[test]
    fn next_to_working_should_be_short_break() {
        let config = TomatoConfig::new(25, 5, 15);
        let session = Session {
            work_count: 0,
            state: State::Working,
            config,
        };
        let next = session.next_state();
        assert_eq!(next, State::ShortBreak);
    }

    #[test]
    fn next_to_repeating_working_should_be_long_break() {
        let config = TomatoConfig::new(25, 5, 15);
        let session = Session {
            work_count: 4,
            state: State::Working,
            config,
        };
        let next = session.next_state();
        assert_eq!(next, State::LongBreak);
    }

    #[test]
    fn next_state_short_break_should_be_working() {
        let config = TomatoConfig::new(25, 5, 15);
        let session = Session {
            work_count: 0,
            state: State::ShortBreak,
            config,
        };
        let next = session.next_state();
        assert_eq!(next, State::Working);
    }

    #[test]
    fn next_state_long_break_should_be_working() {
        let config = TomatoConfig::new(25, 5, 15);
        let session = Session {
            work_count: 0,
            state: State::LongBreak,
            config,
        };
        let next = session.next_state();
        assert_eq!(next, State::Working);
    }
}
