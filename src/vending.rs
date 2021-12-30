use chrono::{DateTime, Local};

use crate::{
    date::MovieDateTime,
    ticket::{GeneralTicket, SeniorTicket, Ticket},
};

pub struct Vending {
    age_type: AgeType,
    movie_date_time: DateTime<Local>,
}

impl Vending {
    pub fn new(movie_date_time: DateTime<Local>) -> Self {
        Vending {
            age_type: AgeType::GENERAL,
            movie_date_time,
        }
    }

    pub fn set_senior(&mut self) {
        self.age_type = AgeType::SENIOR;
    }

    pub fn issue(&self) -> Box<dyn Ticket> {
        if self.age_type == AgeType::SENIOR {
            Box::new(SeniorTicket::new())
        } else {
            let date_time = MovieDateTime::new(self.movie_date_time);
            Box::new(GeneralTicket::new(date_time))
        }
    }
}

#[derive(PartialEq)]
enum AgeType {
    GENERAL,
    SENIOR,
}
