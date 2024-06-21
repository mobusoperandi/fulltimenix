use chrono::{Datelike, Days, NaiveDate, Weekday};

pub struct Friday(NaiveDate);

impl Friday {
    pub fn previous_friday(self) -> Self {
        Self(self.0.checked_sub_days(Days::new(7)).unwrap())
    }
}

impl TryFrom<NaiveDate> for Friday {
    type Error = anyhow::Error;

    fn try_from(date: NaiveDate) -> Result<Self, Self::Error> {
        anyhow::ensure!(
            date.weekday() == Weekday::Fri,
            "provided date is not a Friday"
        );
        Ok(Self(date))
    }
}
