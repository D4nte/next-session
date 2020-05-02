use chrono::{DateTime, Datelike, Duration, NaiveTime, TimeZone, Timelike, Utc, Weekday};

pub struct Session {
    club: String,
    schedule: Schedule,
}

struct Schedule {
    weekday: Weekday,
    time: NaiveTime,
}

impl Schedule {
    pub fn new(weekday: Weekday, time: NaiveTime) -> Schedule {
        Schedule { weekday, time }
    }

    pub fn time_to_next(&self) -> Duration {
        self.time_to_next_from_current(Utc::now())
    }

    pub fn next(&self) -> DateTime<Utc> {
        self.next_from_current(Utc::now())
    }

    pub fn time_to_next_from_current(&self, time: DateTime<Utc>) -> Duration {
        let next = self.next_from_current(time);
        next - time
    }

    /// Calculate when the next session should be based
    /// on the current time.
    pub fn next_from_current(&self, current_time: DateTime<Utc>) -> DateTime<Utc> {
        let weekday = self.weekday.num_days_from_monday();
        let current_weekday = current_time.weekday().num_days_from_monday();

        let days_to = if current_weekday <= weekday {
            weekday - current_weekday
        } else {
            7 + weekday - current_weekday
        };

        let mut next = Utc
            .ymd(
                current_time.year(),
                current_time.month(),
                current_time.day(),
            )
            .and_hms(self.time.hour(), self.time.minute(), self.time.second());
        next = next + Duration::days(days_to as i64);
        if next < current_time {
            // Session was today
            next = next + Duration::days(7)
        }
        next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_session_tomorrow_same_time() {
        // Saturday 2nd of May
        let current_time = Utc.ymd(2020, 5, 2).and_hms(10, 00, 00);

        let schedule = Schedule::new(Weekday::Sun, NaiveTime::from_hms(10, 0, 0));

        assert_eq!(
            schedule.next_from_current(current_time),
            Utc.ymd(2020, 5, 3).and_hms(10, 00, 00)
        );
    }

    #[test]
    fn next_session_in_six_days_same_time() {
        // Saturday 2nd of May
        let current_time = Utc.ymd(2020, 5, 2).and_hms(10, 00, 00);

        let schedule = Schedule::new(Weekday::Fri, NaiveTime::from_hms(10, 0, 0));

        assert_eq!(
            schedule.next_from_current(current_time),
            Utc.ymd(2020, 5, 8).and_hms(10, 00, 00)
        );
    }

    #[test]
    fn next_session_in_six_days_twenty_three_hours() {
        // Saturday 2nd of May
        let current_time = Utc.ymd(2020, 5, 2).and_hms(10, 00, 00);

        let schedule = Schedule::new(Weekday::Sat, NaiveTime::from_hms(9, 0, 0));

        assert_eq!(
            schedule.next_from_current(current_time),
            Utc.ymd(2020, 5, 9).and_hms(9, 00, 00)
        );
    }

    #[test]
    fn one_day_to_next_session() {
        // Saturday 2nd of May
        let current_time = Utc.ymd(2020, 5, 2).and_hms(10, 00, 00);

        let schedule = Schedule::new(Weekday::Sun, NaiveTime::from_hms(10, 0, 0));

        assert_eq!(
            schedule.time_to_next_from_current(current_time),
            Duration::days(1)
        );
    }

    #[test]
    fn one_hour_to_next_session() {
        // Saturday 2nd of May
        let current_time = Utc.ymd(2020, 5, 2).and_hms(10, 00, 00);

        let schedule = Schedule::new(Weekday::Sat, NaiveTime::from_hms(11, 0, 0));

        assert_eq!(
            schedule.time_to_next_from_current(current_time),
            Duration::hours(1)
        );
    }

    #[test]
    fn three_days_to_next_session_next_week() {
        // Saturday 2nd of May
        let current_time = Utc.ymd(2020, 5, 2).and_hms(10, 00, 00);

        let schedule = Schedule::new(Weekday::Tue, NaiveTime::from_hms(10, 0, 0));

        assert_eq!(
            schedule.time_to_next_from_current(current_time),
            Duration::days(3)
        );
    }

    #[test]
    fn six_days_twenty_three_hours_to_next_session() {
        // Saturday 2nd of May
        let current_time = Utc.ymd(2020, 5, 2).and_hms(10, 00, 00);

        let schedule = Schedule::new(Weekday::Sat, NaiveTime::from_hms(09, 0, 0));

        assert_eq!(
            schedule.time_to_next_from_current(current_time),
            Duration::days(6) + Duration::hours(23)
        );
    }
}
