#[cfg(test)]
mod default_price {
    use movie_ticket_price_modeling::vending::Vending;

    use crate::utils::*;

    #[test]
    fn when_movie_time_is_before_20h_then_1800() {
        for &movie_date in get_before_20h_date_times().iter() {
            let vending = Vending::new(movie_date);

            let actual = vending.issue();

            assert_eq!(1800, actual.price());
        }
    }

    #[test]
    fn when_movie_time_is_after_20h_then_1300() {
        for &movie_date in get_after_20h_date_times().iter() {
            let vending = Vending::new(movie_date);

            let actual = vending.issue();

            assert_eq!(1300, actual.price());
        }
    }

    #[test]
    fn when_movie_day_then_1100() {
        for &movie_date in get_movie_day_date_times().iter() {
            let vending = Vending::new(movie_date);

            let actual = vending.issue();

            assert_eq!(1100, actual.price());
        }
    }
}

mod senior {
    use movie_ticket_price_modeling::vending::Vending;

    use crate::utils::get_movie_date_times;

    #[test]
    fn when_age_is_more_than_70_then_1100() {
        for &movie_date_time in get_movie_date_times().iter() {
            let mut vending = Vending::new(movie_date_time);
            vending.set_senior();

            let ticket = vending.issue();

            assert_eq!(1100, ticket.price());
        }
    }
}

mod students {
    use movie_ticket_price_modeling::vending::Vending;

    use crate::utils::*;

    #[test]
    fn when_junior_high_or_high_school_students_then_1000() {
        for &movie_date_time in get_movie_date_times().iter() {
            let mut vending = Vending::new(movie_date_time);
            vending.set_junior_high_or_high_school_students();

            let ticket = vending.issue();

            assert_eq!(1000, ticket.price());
        }
    }

    #[test]
    fn when_college_or_vocational_school_students_on_before_20h_then_1500() {
        for &movie_date_time in get_before_20h_date_times().iter() {
            let mut vending = Vending::new(movie_date_time);
            vending.set_college_or_vocational_school_students();

            let ticket = vending.issue();

            assert_eq!(1500, ticket.price());
        }
    }

    #[test]
    fn when_college_or_vocational_school_students_on_after_20h_then_1300() {
        for &movie_date_time in get_after_20h_date_times().iter() {
            let mut vending = Vending::new(movie_date_time);
            vending.set_college_or_vocational_school_students();

            let ticket = vending.issue();

            assert_eq!(1300, ticket.price());
        }
    }

    #[test]
    fn when_college_or_vocational_school_students_on_movie_day_then_1100() {
        for &movie_date in get_movie_day_date_times().iter() {
            let mut vending = Vending::new(movie_date);
            vending.set_college_or_vocational_school_students();

            let actual = vending.issue();

            assert_eq!(1100, actual.price());
        }
    }

    #[test]
    fn when_primary_school_students_and_younger_then_1000() {
        for &movie_date_time in get_movie_date_times().iter() {
            let mut vending = Vending::new(movie_date_time);
            vending.set_primary_school_students_and_younger();

            let ticket = vending.issue();

            assert_eq!(1000, ticket.price());
        }
    }
}

mod member {
    use movie_ticket_price_modeling::vending::Vending;

    use crate::utils::*;

    #[test]
    fn when_member_on_weekday_then_1000() {
        for &movie_date_time in get_weekday_date_times().iter() {
            let mut vending = Vending::new(movie_date_time);
            vending.set_member();

            let ticket = vending.issue();

            assert_eq!(1000, ticket.price());
        }
    }

    #[test]
    fn when_member_on_weekend_before_20h_then_1300() {
        for &movie_date_time in get_weekend_before_20h_date_times().iter() {
            let mut vending = Vending::new(movie_date_time);
            vending.set_member();

            let ticket = vending.issue();

            assert_eq!(1300, ticket.price());
        }
    }

    #[test]
    fn when_member_on_weekend_after_20h_then_1000() {
        for &movie_date_time in get_weekend_after_20h_date_times().iter() {
            let mut vending = Vending::new(movie_date_time);
            vending.set_member();

            let ticket = vending.issue();

            assert_eq!(1000, ticket.price());
        }
    }

    #[test]
    fn when_member_on_movie_day_then_1100() {
        for &movie_date_time in get_movie_day_date_times().iter() {
            let mut vending = Vending::new(movie_date_time);
            vending.set_member();

            let ticket = vending.issue();

            assert_eq!(1100, ticket.price());
        }
    }

    #[test]
    fn when_senior_member_then_1000() {
        for &movie_date_time in get_movie_date_times().iter() {
            let mut vending = Vending::new(movie_date_time);
            vending.set_senior_member();

            let ticket = vending.issue();

            assert_eq!(1000, ticket.price());
        }
    }
}

mod utils {
    use chrono::{DateTime, Local, TimeZone};

    pub fn get_movie_date_times() -> Vec<DateTime<Local>> {
        let mut date_times = get_before_20h_date_times();
        date_times.append(&mut get_after_20h_date_times());
        date_times
    }

    pub fn get_weekday_before_20h_date_times() -> Vec<DateTime<Local>> {
        vec![
            Local.ymd(2021, 12, 27).and_hms(19, 59, 59),
            Local.ymd(2021, 12, 28).and_hms(0, 0, 0),
            Local.ymd(2021, 12, 29).and_hms(11, 23, 45),
            Local.ymd(2021, 12, 30).and_hms(18, 00, 20),
            Local.ymd(2021, 12, 31).and_hms(1, 2, 3),
        ]
    }

    pub fn get_weekday_after_20h_date_times() -> Vec<DateTime<Local>> {
        vec![
            Local.ymd(2022, 1, 3).and_hms(20, 00, 00),
            Local.ymd(2022, 1, 4).and_hms(20, 00, 01),
            Local.ymd(2022, 1, 5).and_hms(23, 59, 59),
            Local.ymd(2022, 1, 6).and_hms(21, 43, 56),
            Local.ymd(2022, 1, 7).and_hms(22, 05, 17),
        ]
    }

    pub fn get_weekend_before_20h_date_times() -> Vec<DateTime<Local>> {
        vec![
            Local.ymd(2021, 12, 25).and_hms(19, 59, 59),
            Local.ymd(2021, 12, 26).and_hms(0, 0, 0),
        ]
    }

    pub fn get_weekend_after_20h_date_times() -> Vec<DateTime<Local>> {
        vec![
            Local.ymd(2022, 1, 8).and_hms(20, 00, 00),
            Local.ymd(2022, 1, 2).and_hms(23, 59, 59),
        ]
    }

    pub fn get_movie_day_date_times() -> Vec<DateTime<Local>> {
        vec![
            Local.ymd(2021, 12, 1).and_hms(0, 00, 00),
            Local.ymd(2022, 1, 1).and_hms(23, 59, 59),
        ]
    }

    pub fn get_before_20h_date_times() -> Vec<DateTime<Local>> {
        let mut date_times = get_weekday_before_20h_date_times();
        date_times.append(&mut get_weekend_before_20h_date_times());
        date_times
    }

    pub fn get_after_20h_date_times() -> Vec<DateTime<Local>> {
        let mut date_times = get_weekday_after_20h_date_times();
        date_times.append(&mut get_weekend_after_20h_date_times());
        date_times
    }

    pub fn get_weekday_date_times() -> Vec<DateTime<Local>> {
        let mut date_times = get_weekday_before_20h_date_times();
        date_times.append(&mut get_weekday_after_20h_date_times());
        date_times
    }
}
