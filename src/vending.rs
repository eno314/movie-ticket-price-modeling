use chrono::{DateTime, Local};

use crate::{kinds, ticket::*};

pub struct Vending {
    age: kinds::Age,
    movie_date_time: DateTime<Local>,
}

impl Vending {
    pub fn new(movie_date_time: DateTime<Local>) -> Self {
        Vending {
            age: kinds::Age::General,
            movie_date_time,
        }
    }

    pub fn set_senior(&mut self) {
        self.age = kinds::Age::Senior;
    }

    pub fn set_junior_high_or_high_school_students(&mut self) {
        self.age = kinds::Age::JuniorHighOrHighSchoolStudents
    }

    pub fn set_college_or_vocational_school_students(&mut self) {
        self.age = kinds::Age::CollegeOrVocationalSchoolStudents
    }

    pub fn issue(&self) -> Box<dyn Ticket> {
        match self.age {
            kinds::Age::CollegeOrVocationalSchoolStudents => Box::new(
                create_college_or_vocational_school_students(self.movie_date_time),
            ),
            kinds::Age::General => Box::new(create_general_ticket(self.movie_date_time)),
            kinds::Age::JuniorHighOrHighSchoolStudents => {
                Box::new(create_junior_high_or_high_school_students_ticket())
            }
            kinds::Age::Senior => Box::new(create_senior_ticket()),
        }
    }
}
