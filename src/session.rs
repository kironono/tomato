use core::time;
use std::thread;

use indicatif::ProgressBar;

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
                    process_state(self.config.work_time, "Working..");
                    self.work_count += 1;
                }

                State::LongBreak => process_state(self.config.long_break_time, "Long Breaking.."),
                State::ShortBreak => {
                    process_state(self.config.short_break_time, "Short Breaking..")
                }
            }
            self.next();
        }
    }
}

fn process_state(duration: u64, state_msg: &str) {
    let duration_second = duration;

    let progress = ProgressBar::new(duration_second);

    println!("{}", state_msg);
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
    fn next_state_working_after_long_break() {
        let config = TomatoConfig::new(25, 5, 15);
        let mut session = Session {
            work_count: 0,
            state: State::Working,
            config,
        };
        session.work_count = 4;
        let next = session.next_state();
        assert_eq!(next, State::LongBreak);
    }

    #[test]
    fn next_state_working_after_short_break() {
        let config = TomatoConfig::new(25, 5, 15);
        let session = Session {
            work_count: 0,
            state: State::Working,
            config,
        };
        let next = session.next_state();
        assert_eq!(next, State::ShortBreak);
    }
}
