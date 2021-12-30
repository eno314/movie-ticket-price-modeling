use crate::date::MovieDateTime;

pub trait Ticket {
    fn price(&self) -> u32;
}

pub struct GeneralTicket {
    date_time: MovieDateTime,
}

impl GeneralTicket {
    pub fn new(date_time: MovieDateTime) -> impl Ticket {
        GeneralTicket { date_time }
    }
}

impl Ticket for GeneralTicket {
    fn price(&self) -> u32 {
        if self.date_time.is_movie_day() {
            return 1100;
        }
        if self.date_time.is_late_show() {
            return 1300;
        }
        1800
    }
}

pub struct SeniorTicket;

impl SeniorTicket {
    pub fn new() -> impl Ticket {
        SeniorTicket {}
    }
}

impl Ticket for SeniorTicket {
    fn price(&self) -> u32 {
        1100
    }
}
