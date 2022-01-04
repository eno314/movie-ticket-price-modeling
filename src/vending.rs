use chrono::{DateTime, Local};

use crate::ticket::*;

enum TicketKinds {
    CollegeOrVocationalSchoolStudents,
    Disabilities,
    General,
    HighSchoolStudentsAndYoungerWithDisabilities,
    JuniorHighOrHighSchoolStudents,
    Member,
    PrimarySchoolStudentsAndYounger,
    Senior,
    SeniorMember,
}

pub struct Vending {
    ticket_kinds: TicketKinds,
    movie_date_time: DateTime<Local>,
}

impl Vending {
    pub fn new(movie_date_time: DateTime<Local>) -> Self {
        Vending {
            ticket_kinds: TicketKinds::General,
            movie_date_time,
        }
    }

    pub fn set_college_or_vocational_school_students(&mut self) {
        self.ticket_kinds = TicketKinds::CollegeOrVocationalSchoolStudents;
    }

    pub fn set_disabilities(&mut self) {
        self.ticket_kinds = TicketKinds::Disabilities;
    }

    pub fn set_high_school_students_and_younger_with_disabilities(&mut self) {
        self.ticket_kinds = TicketKinds::HighSchoolStudentsAndYoungerWithDisabilities;
    }

    pub fn set_junior_high_or_high_school_students(&mut self) {
        self.ticket_kinds = TicketKinds::JuniorHighOrHighSchoolStudents;
    }

    pub fn set_member(&mut self) {
        self.ticket_kinds = TicketKinds::Member;
    }

    pub fn set_primary_school_students_and_younger(&mut self) {
        self.ticket_kinds = TicketKinds::PrimarySchoolStudentsAndYounger;
    }

    pub fn set_senior(&mut self) {
        self.ticket_kinds = TicketKinds::Senior;
    }

    pub fn set_senior_member(&mut self) {
        self.ticket_kinds = TicketKinds::SeniorMember;
    }

    pub fn issue(&self) -> Box<dyn Ticket> {
        match self.ticket_kinds {
            TicketKinds::CollegeOrVocationalSchoolStudents => Box::new(
                create_college_or_vocational_school_students(self.movie_date_time),
            ),
            TicketKinds::Disabilities => Box::new(create_disabilities_ticket()),
            TicketKinds::General => Box::new(create_general_ticket(self.movie_date_time)),
            TicketKinds::HighSchoolStudentsAndYoungerWithDisabilities => {
                Box::new(create_high_school_students_and_younger_with_disabilities_ticket())
            }
            TicketKinds::JuniorHighOrHighSchoolStudents
            | TicketKinds::PrimarySchoolStudentsAndYounger => {
                Box::new(create_high_school_students_and_younger_ticket())
            }
            TicketKinds::Member => Box::new(create_member_ticket(self.movie_date_time)),
            TicketKinds::Senior => Box::new(create_senior_ticket()),
            TicketKinds::SeniorMember => Box::new(create_senior_member_ticket()),
        }
    }
}
