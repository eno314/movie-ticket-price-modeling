use crate::date::MovieDateTime;

use super::Ticket;

pub struct MiCardTicket {
    date_time: MovieDateTime,
}

impl MiCardTicket {
    pub fn new(date_time: MovieDateTime) -> impl Ticket {
        MiCardTicket { date_time }
    }
}

impl Ticket for MiCardTicket {
    fn price(&self) -> u32 {
        if self.date_time.is_late_show() {
            return 1300;
        }
        1600
    }
}
