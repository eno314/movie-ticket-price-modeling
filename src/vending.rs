use chrono::{DateTime, Local};

use crate::{
    date::MovieDateTime,
    ticket::{GeneralTicket, SeniorTicket, Ticket},
};

pub struct Vending {
    user_age: u32,
    movie_date_time: DateTime<Local>,
}

impl Vending {
    pub fn new(user_age: u32, movie_date_time: DateTime<Local>) -> Self {
        Vending {
            user_age,
            movie_date_time,
        }
    }

    pub fn issue(&self) -> Box<dyn Ticket> {
        if self.user_age >= 70 {
            Box::new(SeniorTicket::new())
        } else {
            let date_time = MovieDateTime::new(self.movie_date_time);
            Box::new(GeneralTicket::new(date_time))
        }
    }
}
