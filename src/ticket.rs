use crate::date::MovieDateTime;

pub struct Ticket {
    date_time: MovieDateTime,
}

impl Ticket {
    pub fn new(date_time: MovieDateTime) -> Self {
        Ticket { date_time }
    }

    pub fn price(&self) -> u32 {
        if self.date_time.is_movie_day() {
            return 1100;
        }
        if self.date_time.is_late_show() {
            return 1300;
        }
        1800
    }
}
