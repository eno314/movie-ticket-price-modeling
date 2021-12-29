use chrono::{DateTime, Local};

use crate::{date::MovieDateTime, ticket::Ticket};

pub struct Vending {
    movie_date_time: DateTime<Local>,
}

impl Vending {
    pub fn new(movie_date_time: DateTime<Local>) -> Self {
        Vending { movie_date_time }
    }

    pub fn issue(&self) -> Ticket {
        let date_time = MovieDateTime::new(self.movie_date_time);
        Ticket::new(date_time)
    }
}
