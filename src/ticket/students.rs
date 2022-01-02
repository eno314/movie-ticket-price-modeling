use crate::date::MovieDateTime;

use super::Ticket;

pub struct CollegeOrVocationalSchoolStudentsTicket {
    date_time: MovieDateTime,
}

impl CollegeOrVocationalSchoolStudentsTicket {
    pub fn new(date_time: MovieDateTime) -> impl Ticket {
        CollegeOrVocationalSchoolStudentsTicket { date_time }
    }
}

impl Ticket for CollegeOrVocationalSchoolStudentsTicket {
    fn price(&self) -> u32 {
        if self.date_time.is_movie_day() {
            return 1100;
        }
        if self.date_time.is_late_show() {
            return 1300;
        }
        1500
    }
}

pub struct HighSchoolStudentsAndYoungerTicket;

impl HighSchoolStudentsAndYoungerTicket {
    pub fn new() -> impl Ticket {
        HighSchoolStudentsAndYoungerTicket {}
    }
}

impl Ticket for HighSchoolStudentsAndYoungerTicket {
    fn price(&self) -> u32 {
        1000
    }
}
