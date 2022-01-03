use crate::date::MovieDateTime;

use super::Ticket;

pub struct MemberTicket {
    date_time: MovieDateTime,
}

impl MemberTicket {
    pub fn new(date_time: MovieDateTime) -> impl Ticket {
        MemberTicket { date_time }
    }
}

impl Ticket for MemberTicket {
    fn price(&self) -> u32 {
        if self.date_time.is_movie_day() {
            return 1100;
        }
        if self.date_time.is_weekend() && !self.date_time.is_late_show() {
            return 1300;
        }
        1000
    }
}

pub struct SeniorMemberTicket;

impl SeniorMemberTicket {
    pub fn new() -> impl Ticket {
        SeniorMemberTicket {}
    }
}

impl Ticket for SeniorMemberTicket {
    fn price(&self) -> u32 {
        1000
    }
}
