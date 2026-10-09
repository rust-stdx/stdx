use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::Bound::{Included, Unbounded},
};

use time::{DateTime, TimeZone};

use crate::{ordinal::*, queries::*, time_unit::*};

impl From<Schedule> for String {
    fn from(schedule: Schedule) -> String {
        schedule.source
    }
}

#[derive(Clone, Debug, Eq)]
pub struct Schedule {
    source: String,
    fields: ScheduleFields,
}

impl Schedule {
    pub(crate) fn new(source: String, fields: ScheduleFields) -> Schedule {
        Schedule {
            source,
            fields,
        }
    }

    fn next_after(&self, after: &DateTime) -> Option<DateTime> {
        let mut query = NextAfterQuery::from(after);
        for year in self
            .fields
            .years
            .ordinals()
            .range((Included(query.year_lower_bound()), Unbounded))
            .cloned()
        {
            let month_start = query.month_lower_bound();
            if !self.fields.months.ordinals().contains(&month_start) {
                query.reset_month();
            }
            let month_range = (Included(month_start), Included(Months::inclusive_max()));
            for month in self.fields.months.ordinals().range(month_range).cloned() {
                let day_of_month_start = query.day_of_month_lower_bound();
                if !self.fields.days_of_month.ordinals().contains(&day_of_month_start) {
                    query.reset_day_of_month();
                }
                let day_of_month_end = days_in_month(month, year);
                let day_of_month_range =
                    (Included(day_of_month_start.min(day_of_month_end)), Included(day_of_month_end));

                'day_loop: for day_of_month in self.fields.days_of_month.ordinals().range(day_of_month_range).cloned() {
                    let hour_start = query.hour_lower_bound();
                    if !self.fields.hours.ordinals().contains(&hour_start) {
                        query.reset_hour();
                    }
                    let hour_range = (Included(hour_start), Included(Hours::inclusive_max()));

                    for hour in self.fields.hours.ordinals().range(hour_range).cloned() {
                        let minute_start = query.minute_lower_bound();
                        if !self.fields.minutes.ordinals().contains(&minute_start) {
                            query.reset_minute();
                        }
                        let minute_range = (Included(minute_start), Included(Minutes::inclusive_max()));

                        for minute in self.fields.minutes.ordinals().range(minute_range).cloned() {
                            let second_start = query.second_lower_bound();
                            if !self.fields.seconds.ordinals().contains(&second_start) {
                                query.reset_second();
                            }
                            let second_range = (Included(second_start), Included(Seconds::inclusive_max()));

                            for second in self.fields.seconds.ordinals().range(second_range).cloned() {
                                let candidate = match DateTime::from_parts(
                                    year as i32,
                                    month as u8,
                                    day_of_month as u8,
                                    hour as u8,
                                    minute as u8,
                                    second as u8,
                                    0,
                                    after.timezone(),
                                ) {
                                    Ok(candidate) => candidate,
                                    Err(_) => continue,
                                };
                                if !self
                                    .fields
                                    .days_of_week
                                    .ordinals()
                                    .contains(&((candidate.weekday().number_from_sunday() as Ordinal) + 1))
                                {
                                    continue 'day_loop;
                                }
                                return Some(candidate);
                            }
                            query.reset_minute();
                        } // End of minutes range
                        query.reset_hour();
                    } // End of hours range
                    query.reset_day_of_month();
                } // End of Day of Month range
                query.reset_month();
            } // End of Month range
        }

        // We ran out of dates to try.
        None
    }

    fn prev_from(&self, before: &DateTime) -> Option<DateTime> {
        let mut query = PrevFromQuery::from(before);
        for year in self
            .fields
            .years
            .ordinals()
            .range((Unbounded, Included(query.year_upper_bound())))
            .rev()
            .cloned()
        {
            let month_start = query.month_upper_bound();

            if !self.fields.months.ordinals().contains(&month_start) {
                query.reset_month();
            }
            let month_range = (Included(Months::inclusive_min()), Included(month_start));

            for month in self.fields.months.ordinals().range(month_range).rev().cloned() {
                let day_of_month_end = query.day_of_month_upper_bound();
                if !self.fields.days_of_month.ordinals().contains(&day_of_month_end) {
                    query.reset_day_of_month();
                }

                let day_of_month_end = days_in_month(month, year).min(day_of_month_end);

                let day_of_month_range = (Included(DaysOfMonth::inclusive_min()), Included(day_of_month_end));

                'day_loop: for day_of_month in self
                    .fields
                    .days_of_month
                    .ordinals()
                    .range(day_of_month_range)
                    .rev()
                    .cloned()
                {
                    let hour_start = query.hour_upper_bound();
                    if !self.fields.hours.ordinals().contains(&hour_start) {
                        query.reset_hour();
                    }
                    let hour_range = (Included(Hours::inclusive_min()), Included(hour_start));

                    for hour in self.fields.hours.ordinals().range(hour_range).rev().cloned() {
                        let minute_start = query.minute_upper_bound();
                        if !self.fields.minutes.ordinals().contains(&minute_start) {
                            query.reset_minute();
                        }
                        let minute_range = (Included(Minutes::inclusive_min()), Included(minute_start));

                        for minute in self.fields.minutes.ordinals().range(minute_range).rev().cloned() {
                            let second_start = query.second_upper_bound();
                            if !self.fields.seconds.ordinals().contains(&second_start) {
                                query.reset_second();
                            }
                            let second_range = (Included(Seconds::inclusive_min()), Included(second_start));

                            for second in self.fields.seconds.ordinals().range(second_range).rev().cloned() {
                                let candidate = match DateTime::from_parts(
                                    year as i32,
                                    month as u8,
                                    day_of_month as u8,
                                    hour as u8,
                                    minute as u8,
                                    second as u8,
                                    0,
                                    before.timezone(),
                                ) {
                                    Ok(candidate) => candidate,
                                    Err(_) => continue,
                                };
                                if !self
                                    .fields
                                    .days_of_week
                                    .ordinals()
                                    .contains(&((candidate.weekday().number_from_sunday() as Ordinal) + 1))
                                {
                                    continue 'day_loop;
                                }
                                return Some(candidate);
                            }
                            query.reset_minute();
                        } // End of minutes range
                        query.reset_hour();
                    } // End of hours range
                    query.reset_day_of_month();
                } // End of Day of Month range
                query.reset_month();
            } // End of Month range
        }

        // We ran out of dates to try.
        None
    }

    /// Provides an iterator which will return each DateTime that matches the schedule starting with
    /// the current time if applicable.
    pub fn upcoming(&self, timezone: TimeZone) -> ScheduleIterator<'_> {
        self.after(&DateTime::now_in(timezone))
    }

    /// The same, but with an iterator with a static ownership
    pub fn upcoming_owned(&self, timezone: TimeZone) -> OwnedScheduleIterator {
        self.after_owned(DateTime::now_in(timezone))
    }

    /// Like the `upcoming` method, but allows you to specify a start time other than the present.
    pub fn after(&self, after: &DateTime) -> ScheduleIterator<'_> {
        ScheduleIterator::new(self, after)
    }

    /// The same, but with a static ownership.
    pub fn after_owned(&self, after: DateTime) -> OwnedScheduleIterator {
        OwnedScheduleIterator::new(self.clone(), after)
    }

    pub fn includes(&self, date_time: DateTime) -> bool {
        self.fields.years.includes(date_time.year() as Ordinal)
            && self.fields.months.includes(date_time.month() as Ordinal)
            && self
                .fields
                .days_of_week
                .includes((date_time.weekday().number_from_sunday() as Ordinal) + 1)
            && self.fields.days_of_month.includes(date_time.day() as Ordinal)
            && self.fields.hours.includes(date_time.hour() as Ordinal)
            && self.fields.minutes.includes(date_time.minute() as Ordinal)
            && self.fields.seconds.includes(date_time.second() as Ordinal)
    }

    /// Returns a [TimeUnitSpec] describing the years included in this [Schedule].
    pub fn years(&self) -> &impl TimeUnitSpec {
        &self.fields.years
    }

    /// Returns a [TimeUnitSpec] describing the months of the year included in this [Schedule].
    pub fn months(&self) -> &impl TimeUnitSpec {
        &self.fields.months
    }

    /// Returns a [TimeUnitSpec] describing the days of the month included in this [Schedule].
    pub fn days_of_month(&self) -> &impl TimeUnitSpec {
        &self.fields.days_of_month
    }

    /// Returns a [TimeUnitSpec] describing the days of the week included in this [Schedule].
    pub fn days_of_week(&self) -> &impl TimeUnitSpec {
        &self.fields.days_of_week
    }

    /// Returns a [TimeUnitSpec] describing the hours of the day included in this [Schedule].
    pub fn hours(&self) -> &impl TimeUnitSpec {
        &self.fields.hours
    }

    /// Returns a [TimeUnitSpec] describing the minutes of the hour included in this [Schedule].
    pub fn minutes(&self) -> &impl TimeUnitSpec {
        &self.fields.minutes
    }

    /// Returns a [TimeUnitSpec] describing the seconds of the minute included in this [Schedule].
    pub fn seconds(&self) -> &impl TimeUnitSpec {
        &self.fields.seconds
    }

    pub fn timeunitspec_eq(&self, other: &Schedule) -> bool {
        self.fields == other.fields
    }
}

impl Display for Schedule {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.source)
    }
}

impl PartialEq for Schedule {
    fn eq(&self, other: &Schedule) -> bool {
        self.source == other.source
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScheduleFields {
    years: Years,
    days_of_week: DaysOfWeek,
    months: Months,
    days_of_month: DaysOfMonth,
    hours: Hours,
    minutes: Minutes,
    seconds: Seconds,
}

impl ScheduleFields {
    pub(crate) fn new(
        seconds: Seconds,
        minutes: Minutes,
        hours: Hours,
        days_of_month: DaysOfMonth,
        months: Months,
        days_of_week: DaysOfWeek,
        years: Years,
    ) -> ScheduleFields {
        ScheduleFields {
            years,
            days_of_week,
            months,
            days_of_month,
            hours,
            minutes,
            seconds,
        }
    }
}

pub struct ScheduleIterator<'a> {
    schedule: &'a Schedule,
    previous_datetime: Option<DateTime>,
}
//TODO: Cutoff datetime?

impl<'a> ScheduleIterator<'a> {
    fn new(schedule: &'a Schedule, starting_datetime: &DateTime) -> Self {
        ScheduleIterator {
            schedule,
            previous_datetime: Some(*starting_datetime),
        }
    }
}

impl Iterator for ScheduleIterator<'_> {
    type Item = DateTime;

    fn next(&mut self) -> Option<DateTime> {
        let previous = self.previous_datetime.take()?;

        if let Some(next) = self.schedule.next_after(&previous) {
            self.previous_datetime = Some(next);
            Some(next)
        } else {
            None
        }
    }
}

impl DoubleEndedIterator for ScheduleIterator<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let previous = self.previous_datetime.take()?;

        if let Some(prev) = self.schedule.prev_from(&previous) {
            self.previous_datetime = Some(prev);
            Some(prev)
        } else {
            None
        }
    }
}

/// A `ScheduleIterator` with a static lifetime.
pub struct OwnedScheduleIterator {
    schedule: Schedule,
    previous_datetime: Option<DateTime>,
}

impl OwnedScheduleIterator {
    pub fn new(schedule: Schedule, starting_datetime: DateTime) -> Self {
        Self {
            schedule,
            previous_datetime: Some(starting_datetime),
        }
    }
}

impl Iterator for OwnedScheduleIterator {
    type Item = DateTime;

    fn next(&mut self) -> Option<DateTime> {
        let previous = self.previous_datetime.take()?;

        if let Some(next) = self.schedule.next_after(&previous) {
            self.previous_datetime = Some(next);
            Some(next)
        } else {
            None
        }
    }
}

impl DoubleEndedIterator for OwnedScheduleIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        let previous = self.previous_datetime.take()?;

        if let Some(prev) = self.schedule.prev_from(&previous) {
            self.previous_datetime = Some(prev);
            Some(prev)
        } else {
            None
        }
    }
}

fn is_leap_year(year: Ordinal) -> bool {
    let by_four = year % 4 == 0;
    let by_hundred = year % 100 == 0;
    let by_four_hundred = year % 400 == 0;
    by_four && ((!by_hundred) || by_four_hundred)
}

fn days_in_month(month: Ordinal, year: Ordinal) -> u32 {
    let is_leap_year = is_leap_year(year);
    match month {
        9 | 4 | 6 | 11 => 30,
        2 if is_leap_year => 29,
        2 => 28,
        _ => 31,
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_next_and_prev_from() {
        let expression = "0 5,13,40-42 17 1 Jan *";
        let schedule = Schedule::from_str(expression).unwrap();

        let next = schedule.next_after(&DateTime::now_in(TimeZone::UTC));
        println!("NEXT AFTER for {} {:?}", expression, next);
        assert!(next.is_some());

        let next2 = schedule.next_after(&next.unwrap());
        println!("NEXT2 AFTER for {} {:?}", expression, next2);
        assert!(next2.is_some());

        let prev = schedule.prev_from(&next2.unwrap());
        println!("PREV FROM for {} {:?}", expression, prev);
        assert!(prev.is_some());
        assert_eq!(prev, next);
    }

    #[test]
    fn test_prev_from() {
        let expression = "0 5,13,40-42 17 1 Jan *";
        let schedule = Schedule::from_str(expression).unwrap();
        let prev = schedule.prev_from(&DateTime::now_in(TimeZone::UTC));
        println!("PREV FROM for {} {:?}", expression, prev);
        assert!(prev.is_some());
    }

    #[test]
    fn test_next_after() {
        let expression = "0 5,13,40-42 17 1 Jan *";
        let schedule = Schedule::from_str(expression).unwrap();
        let next = schedule.next_after(&DateTime::now_in(TimeZone::UTC));
        println!("NEXT AFTER for {} {:?}", expression, next);
        assert!(next.is_some());
    }

    #[test]
    fn test_upcoming_utc() {
        let expression = "0 0,30 0,6,12,18 1,15 Jan-March Thurs";
        let schedule = Schedule::from_str(expression).unwrap();
        let mut upcoming = schedule.upcoming(TimeZone::UTC);
        let next1 = upcoming.next();
        assert!(next1.is_some());
        let next2 = upcoming.next();
        assert!(next2.is_some());
        let next3 = upcoming.next();
        assert!(next3.is_some());
        println!("Upcoming 1 for {} {:?}", expression, next1);
        println!("Upcoming 2 for {} {:?}", expression, next2);
        println!("Upcoming 3 for {} {:?}", expression, next3);
    }

    #[test]
    fn test_upcoming_utc_owned() {
        let expression = "0 0,30 0,6,12,18 1,15 Jan-March Thurs";
        let schedule = Schedule::from_str(expression).unwrap();
        let mut upcoming = schedule.upcoming_owned(TimeZone::UTC);
        let next1 = upcoming.next();
        assert!(next1.is_some());
        let next2 = upcoming.next();
        assert!(next2.is_some());
        let next3 = upcoming.next();
        assert!(next3.is_some());
        println!("Upcoming 1 for {} {:?}", expression, next1);
        println!("Upcoming 2 for {} {:?}", expression, next2);
        println!("Upcoming 3 for {} {:?}", expression, next3);
    }

    #[test]
    fn test_upcoming_rev_utc() {
        let expression = "0 0,30 0,6,12,18 1,15 Jan-March Thurs";
        let schedule = Schedule::from_str(expression).unwrap();
        let mut upcoming = schedule.upcoming(TimeZone::UTC).rev();
        let prev1 = upcoming.next();
        assert!(prev1.is_some());
        let prev2 = upcoming.next();
        assert!(prev2.is_some());
        let prev3 = upcoming.next();
        assert!(prev3.is_some());
        println!("Prev Upcoming 1 for {} {:?}", expression, prev1);
        println!("Prev Upcoming 2 for {} {:?}", expression, prev2);
        println!("Prev Upcoming 3 for {} {:?}", expression, prev3);
    }

    #[test]
    fn test_upcoming_rev_utc_owned() {
        let expression = "0 0,30 0,6,12,18 1,15 Jan-March Thurs";
        let schedule = Schedule::from_str(expression).unwrap();
        let mut upcoming = schedule.upcoming_owned(TimeZone::UTC).rev();
        let prev1 = upcoming.next();
        assert!(prev1.is_some());
        let prev2 = upcoming.next();
        assert!(prev2.is_some());
        let prev3 = upcoming.next();
        assert!(prev3.is_some());
        println!("Prev Upcoming 1 for {} {:?}", expression, prev1);
        println!("Prev Upcoming 2 for {} {:?}", expression, prev2);
        println!("Prev Upcoming 3 for {} {:?}", expression, prev3);
    }

    #[test]
    fn test_upcoming_local() {
        let expression = "0 0,30 0,6,12,18 1,15 Jan-March Thurs";
        let schedule = Schedule::from_str(expression).unwrap();
        let mut upcoming = schedule.upcoming(TimeZone::system().unwrap_or(TimeZone::UTC));
        let next1 = upcoming.next();
        assert!(next1.is_some());
        let next2 = upcoming.next();
        assert!(next2.is_some());
        let next3 = upcoming.next();
        assert!(next3.is_some());
        println!("Upcoming 1 for {} {:?}", expression, next1);
        println!("Upcoming 2 for {} {:?}", expression, next2);
        println!("Upcoming 3 for {} {:?}", expression, next3);
    }

    #[test]
    fn test_schedule_to_string() {
        let expression = "* 1,2,3 * * * *";
        let schedule: Schedule = Schedule::from_str(expression).unwrap();
        let result = String::from(schedule);
        assert_eq!(expression, result);
    }

    #[test]
    fn test_display_schedule() {
        use std::fmt::Write;
        let expression = "@monthly";
        let schedule = Schedule::from_str(expression).unwrap();
        let mut result = String::new();
        write!(result, "{}", schedule).unwrap();
        assert_eq!(expression, result);
    }

    #[test]
    fn test_valid_from_str() {
        let schedule = Schedule::from_str("0 0,30 0,6,12,18 1,15 Jan-March Thurs");
        schedule.unwrap();
    }

    #[test]
    fn test_invalid_from_str() {
        let schedule = Schedule::from_str("cheesecake 0,30 0,6,12,18 1,15 Jan-March Thurs");
        assert!(schedule.is_err());
    }

    // PATCH
    // #[test]
    // fn test_no_panic_on_nonexistent_time_after() {
    //     use chrono::offset::TimeZone;
    //     use chrono_tz::Tz;

    //     let schedule_tz: Tz = "Europe/London".parse().unwrap();
    //     let dt = schedule_tz
    //         .ymd(2019, 10, 27)
    //         .and_hms(0, 3, 29)
    //         .checked_add_signed(chrono::Duration::hours(1)) // puts it in the middle of the DST transition
    //         .unwrap();
    //     let schedule = Schedule::from_str("* * * * * Sat,Sun *").unwrap();
    //     let next = schedule.after(&dt).next().unwrap();
    //     assert!(next > dt); // test is ensuring line above does not panic
    // }

    // PATCH
    // #[test]
    // fn test_no_panic_on_nonexistent_time_before() {
    //     use chrono::offset::TimeZone;
    //     use chrono_tz::Tz;

    //     let schedule_tz: Tz = "Europe/London".parse().unwrap();
    //     let dt = schedule_tz
    //         .ymd(2019, 10, 27)
    //         .and_hms(0, 3, 29)
    //         .checked_add_signed(chrono::Duration::hours(1)) // puts it in the middle of the DST transition
    //         .unwrap();
    //     let schedule = Schedule::from_str("* * * * * Sat,Sun *").unwrap();
    //     let prev = schedule.after(&dt).rev().next().unwrap();
    //     assert!(prev < dt); // test is ensuring line above does not panic
    // }

    #[test]
    fn test_no_panic_on_leap_day_time_after() {
        let dt = "2024-02-29T10:00:00.000+08:00".parse::<DateTime>().unwrap();
        let schedule = Schedule::from_str("0 0 0 * * * 2100").unwrap();
        let next = schedule.after(&dt).next().unwrap();
        assert!(next > dt); // test is ensuring line above does not panic
    }

    #[test]
    fn test_time_unit_spec_equality() {
        let schedule_1 = Schedule::from_str("@weekly").unwrap();
        let schedule_2 = Schedule::from_str("0 0 0 * * 1 *").unwrap();
        let schedule_3 = Schedule::from_str("0 0 0 * * 1-7 *").unwrap();
        let schedule_4 = Schedule::from_str("0 0 0 * * * *").unwrap();
        assert_ne!(schedule_1, schedule_2);
        assert!(schedule_1.timeunitspec_eq(&schedule_2));
        assert!(schedule_3.timeunitspec_eq(&schedule_4));
    }
}
