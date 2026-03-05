use chrono::NaiveDate;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SelectionType {
    Single,
    Multiple,
    Range,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CalendarView {
    Day,
    Month,
    Year,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum WeekStartsOn {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl WeekStartsOn {
    /// Ordered weekday header labels (2-letter) starting from this day.
    pub fn ordered_labels(self) -> [&'static str; 7] {
        const DAYS: [&str; 7] = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];
        let start = self.offset() as usize;
        let mut result = [""; 7];
        for i in 0..7 {
            result[i] = DAYS[(start + i) % 7];
        }
        result
    }

    /// Number of days from Sunday (0) to this weekday.
    pub fn offset(self) -> u32 {
        match self {
            Self::Sunday => 0,
            Self::Monday => 1,
            Self::Tuesday => 2,
            Self::Wednesday => 3,
            Self::Thursday => 4,
            Self::Friday => 5,
            Self::Saturday => 6,
        }
    }
}

/// The selection value. Not `Copy` because `Multiple` contains a `Vec`.
#[derive(Clone, PartialEq, Debug)]
pub enum CalendarValue {
    Single(Option<NaiveDate>),
    Multiple(Vec<NaiveDate>),
    Range {
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
    },
}

impl CalendarValue {
    pub fn default_for(selection_type: SelectionType) -> Self {
        match selection_type {
            SelectionType::Single => Self::Single(None),
            SelectionType::Multiple => Self::Multiple(vec![]),
            SelectionType::Range => Self::Range {
                start: None,
                end: None,
            },
        }
    }

    pub fn contains(&self, date: NaiveDate) -> bool {
        match self {
            Self::Single(Some(d)) => *d == date,
            Self::Multiple(dates) => dates.contains(&date),
            Self::Range {
                start: Some(s),
                end: Some(e),
            } => date >= *s && date <= *e,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).unwrap()
    }

    // ── WeekStartsOn::ordered_labels ──────────────────────────────────────────

    #[test]
    fn ordered_labels_sunday_start() {
        assert_eq!(
            WeekStartsOn::Sunday.ordered_labels(),
            ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"]
        );
    }

    #[test]
    fn ordered_labels_monday_start() {
        assert_eq!(
            WeekStartsOn::Monday.ordered_labels(),
            ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"]
        );
    }

    #[test]
    fn ordered_labels_saturday_start() {
        assert_eq!(
            WeekStartsOn::Saturday.ordered_labels(),
            ["Sa", "Su", "Mo", "Tu", "We", "Th", "Fr"]
        );
    }

    #[test]
    fn ordered_labels_length_always_seven() {
        for ws in [
            WeekStartsOn::Sunday,
            WeekStartsOn::Monday,
            WeekStartsOn::Tuesday,
            WeekStartsOn::Wednesday,
            WeekStartsOn::Thursday,
            WeekStartsOn::Friday,
            WeekStartsOn::Saturday,
        ] {
            let labels = ws.ordered_labels();
            assert_eq!(labels.len(), 7);
            // All 7 canonical labels must appear exactly once.
            for abbr in ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"] {
                assert_eq!(
                    labels.iter().filter(|&&l| l == abbr).count(),
                    1,
                    "'{abbr}' missing or duplicated for {ws:?}"
                );
            }
        }
    }

    // ── CalendarValue::contains ───────────────────────────────────────────────

    #[test]
    fn contains_single_none_always_false() {
        assert!(!CalendarValue::Single(None).contains(d(2025, 5, 1)));
    }

    #[test]
    fn contains_single_matching_date() {
        assert!(CalendarValue::Single(Some(d(2025, 5, 5))).contains(d(2025, 5, 5)));
    }

    #[test]
    fn contains_single_non_matching_date() {
        assert!(!CalendarValue::Single(Some(d(2025, 5, 5))).contains(d(2025, 5, 6)));
    }

    #[test]
    fn contains_multiple() {
        let val = CalendarValue::Multiple(vec![d(2025, 5, 5), d(2025, 5, 10)]);
        assert!(val.contains(d(2025, 5, 5)));
        assert!(val.contains(d(2025, 5, 10)));
        assert!(!val.contains(d(2025, 5, 7)));
    }

    #[test]
    fn contains_multiple_empty() {
        assert!(!CalendarValue::Multiple(vec![]).contains(d(2025, 5, 1)));
    }

    #[test]
    fn contains_range_interior() {
        let val = CalendarValue::Range {
            start: Some(d(2025, 5, 1)),
            end: Some(d(2025, 5, 31)),
        };
        assert!(val.contains(d(2025, 5, 15)));
    }

    #[test]
    fn contains_range_boundaries_inclusive() {
        let val = CalendarValue::Range {
            start: Some(d(2025, 5, 1)),
            end: Some(d(2025, 5, 31)),
        };
        assert!(val.contains(d(2025, 5, 1)), "start boundary must be inclusive");
        assert!(val.contains(d(2025, 5, 31)), "end boundary must be inclusive");
    }

    #[test]
    fn contains_range_outside() {
        let val = CalendarValue::Range {
            start: Some(d(2025, 5, 1)),
            end: Some(d(2025, 5, 31)),
        };
        assert!(!val.contains(d(2025, 4, 30)));
        assert!(!val.contains(d(2025, 6, 1)));
    }

    #[test]
    fn contains_range_open_end_always_false() {
        // A range with start set but no end chosen yet must not match any date.
        let val = CalendarValue::Range {
            start: Some(d(2025, 5, 1)),
            end: None,
        };
        assert!(!val.contains(d(2025, 5, 1)));
        assert!(!val.contains(d(2025, 5, 15)));
    }

    #[test]
    fn contains_range_no_start_always_false() {
        let val = CalendarValue::Range {
            start: None,
            end: None,
        };
        assert!(!val.contains(d(2025, 5, 1)));
    }
}
