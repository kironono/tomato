pub struct TomatoConfig {
    pub work_time: u64,
    pub short_break_time: u64,
    pub long_break_time: u64,
}

impl TomatoConfig {
    pub fn new(work_time: u64, short_break_time: u64, long_break_time: u64) -> Self {
        TomatoConfig {
            work_time,
            short_break_time,
            long_break_time,
        }
    }
}
