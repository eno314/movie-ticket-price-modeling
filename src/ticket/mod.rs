use chrono::{DateTime, Local};

use crate::date::MovieDateTime;

use self::{
    disabilities::{DisabilitiesTicket, HighSchoolStudentsAndYoungerWithDisabilitiesTicket},
    general::GeneralTicket,
    member::{MemberTicket, SeniorMemberTicket},
    senior::SeniorTicket,
    students::{CollegeOrVocationalSchoolStudentsTicket, HighSchoolStudentsAndYoungerTicket},
};

mod disabilities;
mod general;
mod member;
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

pub fn create_disabilities_ticket() -> impl Ticket {
    DisabilitiesTicket::new()
}

pub fn create_general_ticket(movie_date_time: DateTime<Local>) -> impl Ticket {
    let date_time = MovieDateTime::new(movie_date_time);
    GeneralTicket::new(date_time)
}

pub fn create_high_school_students_and_younger_ticket() -> impl Ticket {
    HighSchoolStudentsAndYoungerTicket::new()
}

pub fn create_high_school_students_and_younger_with_disabilities_ticket() -> impl Ticket {
    HighSchoolStudentsAndYoungerWithDisabilitiesTicket::new()
}

pub fn create_member_ticket(movie_date_time: DateTime<Local>) -> impl Ticket {
    let date_time = MovieDateTime::new(movie_date_time);
    MemberTicket::new(date_time)
}

pub fn create_senior_ticket() -> impl Ticket {
    SeniorTicket::new()
}

pub fn create_senior_member_ticket() -> impl Ticket {
    SeniorMemberTicket::new()
}
