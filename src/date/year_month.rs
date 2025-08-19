use chrono::{Datelike, Month};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct YearMonth {
    pub year: i32,
    pub month: Month,
}

impl Default for YearMonth {
    fn default() -> Self {
        let now: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
        let month = match Month::try_from(u8::try_from(now.month()).unwrap()).ok() {
            Some(month) => month,
            None => Month::January,
        };

        Self {
            year: now.year(),
            month,
        }
    }
}

impl YearMonth {
    pub fn succ(&self) -> Self {
        match self.month {
            Month::December => Self {
                year: self.year + 1,
                month: Month::January,
            },
            _ => Self {
                year: self.year,
                month: self.month.succ(),
            },
        }
    }

    pub fn pred(&self) -> Self {
        match self.month {
            Month::January => Self {
                year: self.year - 1,
                month: Month::December,
            },
            _ => Self {
                year: self.year,
                month: self.month.pred(),
            },
        }
    }

    pub fn file_name(&self) -> String {
        format!(
            "{:04}{:02}_high_score.json",
            self.year,
            self.month.number_from_month()
        )
    }
}
