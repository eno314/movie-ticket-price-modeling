use chrono::{DateTime, Local};

use crate::date::MovieDateTime;

use self::{
    general::GeneralTicket,
    senior::SeniorTicket,
    students::{CollegeOrVocationalSchoolStudentsTicket, HighSchoolStudentsAndYoungerTicket},
};

mod general;
mod senior;
mod students;

pub trait Ticket {
    fn price(&self) -> u32;
}

pub fn create_college_or_vocational_school_students(
    movie_date_time: DateTime<Local>,
) -> impl Ticket {
    let date_time = MovieDateTime::new(movie_date_time);
    CollegeOrVocationalSchoolStudentsTicket::new(date_time)
}

pub fn create_general_ticket(movie_date_time: DateTime<Local>) -> impl Ticket {
    let date_time = MovieDateTime::new(movie_date_time);
    GeneralTicket::new(date_time)
}

pub fn create_high_school_students_and_younger_ticket() -> impl Ticket {
    HighSchoolStudentsAndYoungerTicket::new()
}

pub fn create_senior_ticket() -> impl Ticket {
    SeniorTicket::new()
}
