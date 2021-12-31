use super::Ticket;

pub struct JuniorHighOrHighSchoolStudents;

impl JuniorHighOrHighSchoolStudents {
    pub fn new() -> impl Ticket {
        JuniorHighOrHighSchoolStudents {}
    }
}

impl Ticket for JuniorHighOrHighSchoolStudents {
    fn price(&self) -> u32 {
        1000
    }
}
