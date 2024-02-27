use crate::value::Value;

extern crate iso8601;

pub fn parse_temporal(temp_str: &str) -> Result<Value, String> {
    if temp_str.starts_with("@") {
        let striped = &temp_str[2..temp_str.len() - 1];
        return parse_temporal(striped);
    }

    if let Ok(dt) = iso8601::datetime(temp_str) {
        let cdt = match chrono::DateTime::try_from(dt) {
            Ok(v) => v,
            Err(err) => return Err(format!("{:?}", err)),
        };
        Ok(Value::DateTimeV(cdt))
    } else if let Ok(date) = iso8601::date(temp_str) {
        Ok(Value::DateV(date))
    } else if let Ok(time) = iso8601::time(temp_str) {
        Ok(Value::TimeV(time))
    } else if let Ok(dur) = iso8601::duration(temp_str) {
        Ok(Value::DurationV(dur))
    } else {
        Err("fail to parse temporal value".to_owned())
    }
}

pub fn datetime_add(
    cdt: chrono::DateTime<chrono::FixedOffset>,
    dur: iso8601::Duration,
) -> Result<chrono::DateTime<chrono::FixedOffset>, String> {
    //let cdt = chrono::DateTime::try_from(dt).unwrap();
    if let iso8601::Duration::YMDHMS {
        year,
        month,
        day,
        hour,
        minute,
        second,
        millisecond,
    } = dur
    {
        let secs = second + 60 * minute + 3600 * hour + 86400 * day;
        let cdur = chrono::TimeDelta::new(secs as i64, millisecond * 1000_000).unwrap();
        let mut d0 = cdt
            .checked_add_months(chrono::Months::new(month + year * 12))
            .unwrap();
        d0 = d0.checked_add_days(chrono::Days::new(day as u64)).unwrap();
        d0 = d0.checked_add_signed(cdur).unwrap();
        Ok(d0)
    } else {
        Err("fail to add datetime and duration".to_owned())
    }
}

pub fn datetime_sub(
    cdt: chrono::DateTime<chrono::FixedOffset>,
    dur: iso8601::Duration,
) -> Result<chrono::DateTime<chrono::FixedOffset>, String> {
    //let cdt = chrono::DateTime::try_from(dt).unwrap();
    if let iso8601::Duration::YMDHMS {
        year,
        month,
        day,
        hour,
        minute,
        second,
        millisecond,
    } = dur
    {
        let secs = second + 60 * minute + 3600 * hour + 86400 * day;
        let cdur = chrono::TimeDelta::new(secs as i64, millisecond * 1000_000).unwrap();
        let mut d0 = cdt
            .checked_sub_months(chrono::Months::new(month + year * 12))
            .unwrap();
        d0 = d0.checked_sub_days(chrono::Days::new(day as u64)).unwrap();
        d0 = d0.checked_sub_signed(cdur).unwrap();
        Ok(d0)
    } else {
        Err("fail to add datetime and duration".to_owned())
    }
}

#[cfg(test)]
mod test {
    use super::parse_temporal;
    use crate::value::Value;
    use core::assert_matches::assert_matches;
    extern crate chrono;
    extern crate iso8601;
    use chrono::Datelike;

    #[test]
    fn test_parse_temp_value() {
        assert_matches!(
            parse_temporal(r#"@"2020-04-06T08:00:00@Europe/Berlin""#),
            Ok(Value::DateTimeV(_))
        );
        assert_matches!(
            parse_temporal("2020-04-06T08:00:00@Europe/Berlin"),
            Ok(Value::DateTimeV(_))
        );
        assert_matches!(parse_temporal("PT2H3M"), Ok(Value::DurationV(_)));
    }

    #[test]
    fn test_chrono_functions() {
        let dt = iso8601::datetime("2021-02-27T08:00:00+08:00").unwrap();
        //let dur = iso8601::duration("PT2H3M").unwrap();
        //assert_eq!(dt.date.year, 2020);
        let cdt = chrono::DateTime::try_from(dt).unwrap();
        //let cdur = chrono::Duration::try_from(dur);
        assert_eq!(cdt.month(), 2);
        assert_eq!(cdt.day(), 27);

        let r0 = cdt.checked_add_months(chrono::Months::new(8)).unwrap();
        assert_eq!(
            format!("{}", r0.format("%Y-%m-%dT%H:%M:%S%:z")),
            "2021-10-27T08:00:00+08:00".to_owned()
        );

        let r1 = cdt.checked_add_days(chrono::Days::new(2)).unwrap();
        assert_eq!(
            format!("{}", r1.format("%Y-%m-%dT%H:%M:%S%:z")),
            "2021-03-01T08:00:00+08:00".to_owned()
        );

        let r2 = r0 - r1;
        assert_eq!(r2.to_string(), "PT20736000S");
    }
}
