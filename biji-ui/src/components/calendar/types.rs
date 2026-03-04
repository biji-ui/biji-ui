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
