use chrono::{DateTime, Local, Timelike};

use crate::ticket::Ticket;

pub struct Vending {
    movie_date_time: DateTime<Local>,
}

impl Vending {
    pub fn new(movie_date_time: DateTime<Local>) -> Self {
        Vending { movie_date_time }
    }

    pub fn issue(&self) -> Ticket {
        if self.movie_date_time.hour() >= 20 {
            Ticket::new(true)
        } else {
            Ticket::new(false)
        }
    }
}
