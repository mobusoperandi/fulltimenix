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

#[cfg(test)]
mod test {
    use chrono::NaiveDate;

    use crate::friday::Friday;

    #[test]
    fn try_from_failure() {
        assert!(Friday::try_from(NaiveDate::from_ymd_opt(2024, 6, 20).unwrap()).is_err());
    }

    #[test]
    fn try_from_success() {
        assert!(Friday::try_from(NaiveDate::from_ymd_opt(2024, 6, 21).unwrap()).is_ok());
    }

    #[test]
    fn previous_friday() {
        let friday = Friday::try_from(NaiveDate::from_ymd_opt(2024, 6, 21).unwrap()).unwrap();
        let prev_friday = friday.previous_friday();
        let expected = Friday::try_from(NaiveDate::from_ymd_opt(2024, 6, 14).unwrap()).unwrap();
        assert_eq!(prev_friday, expected);
    }
}
