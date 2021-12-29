pub struct Ticket {
    is_late_show: bool,
}

impl Ticket {
    pub fn new(is_late_show: bool) -> Self {
        Ticket { is_late_show }
    }

    pub fn price(&self) -> u32 {
        if self.is_late_show {
            1300
        } else {
            1800
        }
    }
}
