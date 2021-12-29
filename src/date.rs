use chrono::{DateTime, Datelike, Local, Timelike};

pub struct MovieDateTime {
    date_time: DateTime<Local>,
}

impl MovieDateTime {
    const LATE_SHOW_START_HOUR: u32 = 20;
    const MOVIE_DAY: u32 = 1;

    pub fn new(date_time: DateTime<Local>) -> MovieDateTime {
        MovieDateTime { date_time }
    }

    pub fn is_late_show(&self) -> bool {
        self.date_time.hour() >= Self::LATE_SHOW_START_HOUR
    }

    pub fn is_movie_day(&self) -> bool {
        self.date_time.day() == Self::MOVIE_DAY
    }
}
