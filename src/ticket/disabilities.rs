use super::Ticket;

pub struct DisabilitiesTicket;

impl DisabilitiesTicket {
    pub fn new() -> impl Ticket {
        DisabilitiesTicket {}
    }
}

impl Ticket for DisabilitiesTicket {
    fn price(&self) -> u32 {
        1000
    }
}

pub struct HighSchoolStudentsAndYoungerWithDisabilitiesTicket;

impl HighSchoolStudentsAndYoungerWithDisabilitiesTicket {
    pub fn new() -> impl Ticket {
        HighSchoolStudentsAndYoungerWithDisabilitiesTicket {}
    }
}

impl Ticket for HighSchoolStudentsAndYoungerWithDisabilitiesTicket {
    fn price(&self) -> u32 {
        900
    }
}
