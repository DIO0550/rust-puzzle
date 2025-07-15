use bevy::prelude::Resource;
use chrono::{Datelike, Month};

#[derive(Resource)]
pub struct MonthHighScore {
    pub year: i32,
    pub month: Month,
}

impl MonthHighScore {
    pub fn next(&self) -> Self {
        let new_month = self.month.succ();
        let new_year = match self.month {
            Month::January => self.year + 1,
            _ => self.year,
        };

        let new = MonthHighScore {
            year: new_year,
            month: new_month,
        };

        return new;
    }

    pub fn prev(&self) -> Self {
        let new_month = self.month.pred();
        let new_year = match self.month {
            Month::December => self.year - 1,
            _ => self.year,
        };

        let new = MonthHighScore {
            year: new_year,
            month: new_month,
        };

        return new;
    }
}

impl Default for MonthHighScore {
    fn default() -> Self {
        let now: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
        let month = match Month::try_from(u8::try_from(now.month()).unwrap()).ok() {
            Some(month) => month,
            None => Month::January,
        };

        Self {
            year: now.year(),
            month: Month::from(month),
        }
    }
}
