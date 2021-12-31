use chrono::{DateTime, Local};

use crate::date::MovieDateTime;

use self::{
    general::GeneralTicket, senior::SeniorTicket, students::JuniorHighOrHighSchoolStudents,
};

mod general;
mod senior;
mod students;

pub trait Ticket {
    fn price(&self) -> u32;
}

pub fn create_general_ticket(movie_date_time: DateTime<Local>) -> impl Ticket {
    let date_time = MovieDateTime::new(movie_date_time);
    GeneralTicket::new(date_time)
}

pub fn create_junior_high_or_high_school_students_ticket() -> impl Ticket {
    JuniorHighOrHighSchoolStudents::new()
}

pub fn create_senior_ticket() -> impl Ticket {
    SeniorTicket::new()
}
