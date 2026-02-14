use chrono::{DateTime, Local, NaiveDate, NaiveTime, Utc, NaiveDateTime};
use crate::impl_comparison_for_column;

impl_comparison_for_column!(NaiveDate, NaiveTime, NaiveDateTime, DateTime<Utc>, DateTime<Local>);