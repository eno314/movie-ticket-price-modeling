use super::Ticket;

pub struct SeniorTicket;

impl SeniorTicket {
    pub fn new() -> impl Ticket {
        SeniorTicket {}
    }
}

impl Ticket for SeniorTicket {
    fn price(&self) -> u32 {
        1100
    }
}
