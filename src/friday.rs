use chrono::{Datelike, NaiveDate, Weekday};

pub struct Friday(NaiveDate);

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
