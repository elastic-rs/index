use std::borrow::Borrow;
use std::ops::Deref;
use std::marker::PhantomData;
use std::fmt::{Display, Result as FmtResult, Formatter};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::{Visitor, Error};
use super::format::{DateValue, FormattableDateValue, DateFormat, FormattedDate, ParseError};
use super::formats::ChronoFormat;
use super::mapping::{DateFieldType, DateMapping, DefaultDateMapping};
use private::field::StdField;

pub use chrono::{Datelike, Timelike};

/** A re-export of the `chrono::DateTime` struct with `Utc` timezone. */
pub type ChronoDateTime = DateTime<Utc>;

impl DateFieldType<DefaultDateMapping<ChronoFormat>> for ChronoDateTime {}

impl From<ChronoDateTime> for FormattableDateValue<ChronoFormat> {
    fn from(date: ChronoDateTime) -> Self {
        FormattableDateValue::from(DateValue::from(date))
    }
}

/**
An Elasticsearch `date` type with a required `time` component.

The [format](format/index.html) is provided as a generic parameter.
This struct wraps up a `chrono::DateTime<Utc>` struct, meaning storing time in `Utc` is required.

# Examples

Defining a date using the default format:

```
# use elastic_types::prelude::*;
let date: Date<DefaultDateMapping> = Date::now();
```

Defining a date using a named format:

```
# use elastic_types::prelude::*;
let date: Date<DefaultDateMapping<BasicDateTime>> = Date::now();
```

Accessing the values of a date:

```
# use elastic_types::prelude::*;
let date = Date::<DefaultDateMapping>::now();

//eg: 2010/04/30 13:56:59.372
println!("{}/{}/{} {}:{}:{}.{}",
    date.year(),
    date.month(),
    date.day(),
    date.hour(),
    date.minute(),
    date.second(),
    date.nanosecond() / 1000000
);
```

# Links

- [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/date.html)
*/
#[derive(Debug, Clone, PartialEq)]
pub struct Date<M> where M: DateMapping {
    value: FormattableDateValue<M::Format>,
}

impl<M> Date<M> where M: DateMapping
{
    /**
    Creates a new `Date` from the given `chrono::DateTime<Utc>`.
    
    This function will consume the provided `chrono` date.
    
    # Examples

    Create a `Date` from a `DateValue`:

    ```
    # use elastic_types::prelude::*;
    //Create a DateValue struct
    let date = DateValue::now();
    
    //Give it to the Date struct
    let date: Date<DefaultDateMapping> = Date::new(date);
    ```
    
    If the `Date`s format is `ChronoFormat`, then it can also be created from `chrono::DateTime`:

    ```
    # extern crate elastic_types;
    # use elastic_types::prelude::*;
    # extern crate chrono;
    # fn main() {
    use chrono::Utc;
    
    //Create a chrono DateTime struct
    let chronoDate = Utc::now();
    
    //Give it to the Date struct
    let date: Date<DefaultDateMapping<ChronoFormat>> = Date::new(chronoDate);
    # }
    ```

    If the `Date`s format isn't `ChronoFormat`, then the `chrono::DateTime` will need to be converted into a `DateValue` first.
    This is to make sure you don't accidentally change the format of a date, which could lead to errors at runtime:
    
    ```
    # extern crate elastic_types;
    # use elastic_types::prelude::*;
    # extern crate chrono;
    # fn main() {
    use chrono::Utc;
    
    //Create a chrono DateTime struct
    let chronoDate = Utc::now();
    
    //Give it to the Date struct
    let date: Date<DefaultDateMapping<EpochMillis>> = Date::new(DateValue::from(chronoDate));
    # }
    ```
    */
    pub fn new<I>(date: I) -> Self 
        where I: Into<FormattableDateValue<M::Format>>
    {
        Date {
            value: date.into()
        }
    }

    /**
    Creates an `Date` from the given Utc primitives:
    
    ```
    # use elastic_types::prelude::*;
    let date: Date<DefaultDateMapping> = Date::build(2015, 5, 14, 16, 45, 8, 886);
    ```
    */
    pub fn build(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32, milli: u32) -> Self {
        Date::new(DateValue::build(year, month, day, hour, minute, second, milli))
    }

    /**
    Gets the current system time.
    
    # Examples
    
    ```
    # use elastic_types::prelude::*;
    let date: Date<DefaultDateMapping> = Date::now();
    ```
    */
    pub fn now() -> Self {
        Date::new(DateValue::now())
    }

    /**
    Change the format/mapping of this date.
    
    # Examples
    
    ```
    # use elastic_types::prelude::*;
    //Get the current datetime formatted as basic_date_time
    let date: Date<DefaultDateMapping<BasicDateTime>> = Date::now();
    
    //Change the format to epoch_millis
    let otherdate: Date<DefaultDateMapping<EpochMillis>> = Date::remap(date);
    ```
    */
    pub fn remap<MInto>(date: Date<M>) -> Date<MInto>
        where MInto: DateMapping
    {
        Date::new(DateValue::from(date.value))
    }
}

impl<M> DateFieldType<M> for Date<M>
    where M: DateMapping
{
}

impl<M> From<Date<M>> for FormattableDateValue<M::Format> where M: DateMapping {
    fn from(date: Date<M>) -> Self {
        date.value
    }
}

impl<M> From<FormattableDateValue<M::Format>> for Date<M> where M: DateMapping {
    fn from(date: FormattableDateValue<M::Format>) -> Self {
        Date::new(date)
    }
}

impl<M> From<Date<M>> for DateValue where M: DateMapping {
    fn from(date: Date<M>) -> Self {
        date.value.into()
    }
}

impl<M> From<DateValue> for Date<M> 
    where M: DateMapping 
{
    fn from(value: DateValue) -> Self {
        Date::new(value)
    }
}

impl<M> StdField<ChronoDateTime> for Date<M>
    where M: DateMapping
{
}

impl<M> PartialEq<ChronoDateTime> for Date<M> 
    where M: DateMapping 
{
    fn eq(&self, other: &ChronoDateTime) -> bool {
        PartialEq::eq(&self.value, other)
    }

    fn ne(&self, other: &ChronoDateTime) -> bool {
        PartialEq::ne(&self.value, other)
    }
}

impl<M> PartialEq<Date<M>> for ChronoDateTime 
    where M: DateMapping 
{
    fn eq(&self, other: &Date<M>) -> bool {
        PartialEq::eq(self, &other.value)
    }

    fn ne(&self, other: &Date<M>) -> bool {
        PartialEq::ne(self, &other.value)
    }
}

impl<M> Deref for Date<M> 
    where M: DateMapping 
{
    type Target = ChronoDateTime;
    fn deref(&self) -> &ChronoDateTime {
        self.value.borrow()
    }
}

impl<M> Borrow<ChronoDateTime> for Date<M> 
    where M: DateMapping 
{
    fn borrow(&self) -> &ChronoDateTime {
        self.value.borrow()
    }
}

impl<M> Default for Date<M>
    where M: DateMapping
{
    fn default() -> Self {
        Date::now()
    }
}

impl<M> Display for Date<M>
    where M: DateMapping
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", format(self))
    }
}

impl<M> Serialize for Date<M>
    where M: DateMapping
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.collect_str(&self)
    }
}

impl<'de, M> Deserialize<'de> for Date<M>
    where M: DateMapping
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        #[derive(Default)]
        struct DateTimeVisitor<M>
            where M: DateMapping
        {
            _t: PhantomData<M>,
        }

        impl<'de, M> Visitor<'de> for DateTimeVisitor<M>
            where M: DateMapping
        {
            type Value = Date<M>;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(formatter,
                       "a json string or number containing a formatted date")
            }

            fn visit_str<E>(self, v: &str) -> Result<Date<M>, E>
                where E: Error
            {
                parse(v).map_err(|err| Error::custom(format!("{}", err)))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Date<M>, E>
                where E: Error
            {
                parse(&v.to_string()).map_err(|err| Error::custom(format!("{}", err)))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Date<M>, E>
                where E: Error
            {
                parse(&v.to_string()).map_err(|err| Error::custom(format!("{}", err)))
            }
        }

        deserializer.deserialize_any(DateTimeVisitor::<M>::default())
    }
}

/** A convenience function for formatting a date. */
pub(crate) fn format<'a, M>(date: &'a Date<M>) -> FormattedDate<'a>
    where M: DateMapping
{
    date.value.format()
}

/** A convenience function for parsing a date. */
pub(crate) fn parse<M>(date: &str) -> Result<Date<M>, ParseError> 
    where M: DateMapping 
{
    let parsed = FormattableDateValue::parse(date)?;

    Ok(Date::new(parsed))
}

/**
A [date math](https://www.elastic.co/guide/en/elasticsearch/reference/current/common-options.html#date-math) expression.

Date math expressions start from an anchor date, like the literal `now` or `2017-05-06` and apply math operations to produce a new date value.

# Examples

A date expression for `now` plus 2 days:

```
# use elastic_types::prelude::*;
let expr: DateExpr<BasicDateTime> = DateExpr::now().add_days(2);
```

Which serialises to:

```
# extern crate serde_json;
# #[macro_use] extern crate json_str;
# extern crate elastic_types;
# use elastic_types::prelude::*;
# fn main() {
# let expr: DateExpr<BasicDateTime> = DateExpr::now().add_days(2);
# let ser = serde_json::to_string(&expr).unwrap();
# let expected = json_str!(
"now+2d"
# );
# assert_eq!(expected, ser);
# }
```

A date expression using a concrete date value plus 2 days:

```
# use elastic_types::prelude::*;
let expr: DateExpr<BasicDateTime> = DateExpr::value(DateValue::now()).add_days(2);
```

Which serialises to:

```
# extern crate serde_json;
# #[macro_use] extern crate json_str;
# extern crate elastic_types;
# use elastic_types::prelude::*;
# fn main() {
# let expr: DateExpr<BasicDateTime> = DateExpr::value(DateValue::build(2015, 03, 01, 14, 55, 0, 0)).add_days(2);
# let ser = serde_json::to_string(&expr).unwrap();
# let expected = json_str!(
"20150301T145500.000Z||+2d"
# );
# assert_eq!(expected, ser);
# }
```
*/
#[derive(Debug, Clone, PartialEq)]
pub struct DateExpr<F> {
    anchor: DateExprAnchor<F>,
    ops: Vec<DateExprOp>,
}

impl<F> Display for DateExpr<F> 
    where F: DateFormat
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.anchor.fmt(f)?;

        for op in &self.ops {
            op.fmt(f)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
enum DateExprAnchor<F> {
    Now,
    Value(FormattableDateValue<F>)
}

impl<F> Display for DateExprAnchor<F>
    where F: DateFormat
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            DateExprAnchor::Now => "now".fmt(f),
            DateExprAnchor::Value(ref date) => write!(f, "{}||", date.format()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum DateExprOp {
    Add(usize, DateExprOpUnit),
    Sub(usize, DateExprOpUnit),
    Round(DateExprOpUnit)
}

impl Display for DateExprOp {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            DateExprOp::Add(size, unit) => write!(f, "+{}{}", size, unit),
            DateExprOp::Sub(size, unit) => write!(f, "-{}{}", size, unit),
            DateExprOp::Round(unit) => write!(f, "/{}", unit)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum DateExprOpUnit {
    Year,
    Month,
    Week,
    Day,
    Hour,
    Minute,
    Second
}

impl Display for DateExprOpUnit {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let fmtd = match *self {
            DateExprOpUnit::Year => "y",
            DateExprOpUnit::Month => "M",
            DateExprOpUnit::Week => "w",
            DateExprOpUnit::Day => "d",
            DateExprOpUnit::Hour => "h",
            DateExprOpUnit::Minute => "m",
            DateExprOpUnit::Second => "s"
        };

        fmtd.fmt(f)
    }
}

macro_rules! impl_expr_ops {
    ($op:path, $add:ident, $sub:ident, $round:ident) => (
        /** Add to the anchored date. */
        pub fn $add(mut self, value: usize) -> Self {
            self.ops.push(DateExprOp::Add(value, $op));
            self
        }

        /** Subtract from the anchored date. */
        pub fn $sub(mut self, value: usize) -> Self {
            self.ops.push(DateExprOp::Sub(value, $op));
            self
        }

        /** Round the anchored date. */
        pub fn $round(mut self) -> Self {
            self.ops.push(DateExprOp::Round($op));
            self
        }
    )
}

impl<F> DateExpr<F>
    where F: DateFormat
{
    /**
    Create a new date expression for `now`.
    
    This value is different from `Date::now()` because it doesn't calculate the current date from the system clock.
    It serialises to the literal string `"now"`, which is interpreted by Elasticsearch when indexing.
    */
    pub fn now() -> Self {
        DateExpr {
            anchor: DateExprAnchor::Now,
            ops: Vec::new(),
        }
    }

    /** 
    Create a new date expression from a concrete date value.

    If the input is a `DateValue`, then it'll use any format specified on the `DateExpr`.
    If the input is a `Date` or `chrono::DateTime`, then the format on its mapping must match the format expected by the `DateExpr`.

    # Examples

    Create a date expression from a `chrono::DateTime`:

    ```
    # use elastic_types::prelude::*;
    let date = DateValue::now();

    // The format annotation `EpochMillis` is required
    let expr: DateExpr<EpochMillis> = DateExpr::value(date);
    ```

    Create a date expression from a `Date`:

    ```
    # use elastic_types::prelude::*;
    let date: Date<DefaultDateMapping<EpochMillis>> = Date::now();

    // The format `EpochMillis` is inferred
    let expr = DateExpr::value(date);
    ```

    Attempting to create a date expression from a `Date` with a different format will fail to compile:

    ```
    # use elastic_types::prelude::*;
    let date: Date<DefaultDateMapping<BasicDateTime>> = Date::now();

    // Error: expected struct `EpochMillis`, found struct `BasicDateTime`
    # let date = DateValue::from(date);
    let expr: DateExpr<EpochMillis> = DateExpr::value(date);
    ```

    This is to ensure formats aren't silently converted when they shouldn't be, leading to runtime errors in Elasticsearch.
    Convert the date into a `DateValue` first:

    ```
    # use elastic_types::prelude::*;
    let date: Date<DefaultDateMapping<BasicDateTime>> = Date::now();

    let expr: DateExpr<EpochMillis> = DateExpr::value(DateValue::from(date));
    ```
    */
    pub fn value<D>(date: D) -> Self
        where D: Into<FormattableDateValue<F>>
    {
        let date = date.into();
        
        DateExpr {
            anchor: DateExprAnchor::Value(date),
            ops: Vec::new(),
        }
    }

    impl_expr_ops!(DateExprOpUnit::Year, add_years, sub_years, round_year);
    impl_expr_ops!(DateExprOpUnit::Month, add_months, sub_months, round_month);
    impl_expr_ops!(DateExprOpUnit::Week, add_weeks, sub_weeks, round_week);
    impl_expr_ops!(DateExprOpUnit::Day, add_days, sub_days, round_day);
    impl_expr_ops!(DateExprOpUnit::Hour, add_hours, sub_hours, round_hour);
    impl_expr_ops!(DateExprOpUnit::Minute, add_minutes, sub_minutes, round_minute);
    impl_expr_ops!(DateExprOpUnit::Second, add_seconds, sub_seconds, round_second);
}

impl<F> Serialize for DateExpr<F> 
    where F: DateFormat
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.collect_str(&self)
    }
}

#[cfg(test)]
mod tests {
    use serde_json;
    use chrono;
    use chrono::offset::TimeZone;

    use prelude::*;

    #[derive(ElasticDateFormat, Default, Clone)]
    #[elastic(date_format="yyyy/MM/dd HH:mm:ss", date_format_name="test_date_1")]
    pub struct NamedDateFormat;

    #[derive(ElasticDateFormat, Default, Clone, Copy)]
    #[elastic(date_format="yyyyMMdd")]
    pub struct UnNamedDateFormat;

    #[test]
    fn date_format_uses_name_if_supplied() {
        assert_eq!("test_date_1", NamedDateFormat::name());
    }

    #[test]
    fn date_format_uses_format_if_name_not_supplied() {
        assert_eq!("yyyyMMdd", UnNamedDateFormat::name());
    }

    #[test]
    fn dates_should_use_chrono_format() {
        let dt = DateValue::build(2015, 05, 13, 0, 0, 0, 0);

        let dt = Date::<DefaultDateMapping<NamedDateFormat>>::new(dt.clone());
        let actual = format(&dt).to_string();

        assert_eq!("2015/05/13 00:00:00", actual);
    }

    #[test]
    fn dates_should_use_es_format() {
        let dt = DateValue::build(2015, 05, 13, 0, 0, 0, 0);

        let dt = Date::<DefaultDateMapping<UnNamedDateFormat>>::new(dt.clone());
        let actual = format(&dt).to_string();

        assert_eq!("20150513", actual);
    }

    #[test]
    fn can_change_date_mapping() {
        fn takes_epoch_millis(_: Date<DefaultDateMapping<EpochMillis>>) -> bool {
            true
        }

        let date: Date<DefaultDateMapping<BasicDateTime>> = Date::now();

        assert!(takes_epoch_millis(Date::remap(date)));
    }

    #[test]
    fn can_build_date_from_value() {
        let date: Date<DefaultDateMapping> = Date::new(DateValue::build(2015, 05, 13, 0, 0, 0, 0));

        assert_eq!((2015, 5, 13, 0, 0, 0),
                   (date.year(), date.month(), date.day(), date.hour(), date.minute(), date.second()));
    }

    #[test]
    fn can_build_date_from_chrono() {
        let date = chrono::Utc.datetime_from_str("13/05/2015 00:00:00", "%d/%m/%Y %H:%M:%S").unwrap();

        let date: Date<DefaultDateMapping<ChronoFormat>> = Date::new(date);

        assert_eq!((2015, 5, 13, 0, 0, 0),
                   (date.year(), date.month(), date.day(), date.hour(), date.minute(), date.second()));
    }

    #[test]
    fn can_build_date_from_prim() {
        let date: Date<DefaultDateMapping> = Date::build(2015, 5, 13, 0, 0, 0, 0);

        assert_eq!((2015, 5, 13, 0, 0, 0),
                   (date.year(), date.month(), date.day(), date.hour(), date.minute(), date.second()));
    }

    #[test]
    fn serialise_elastic_date() {
        let date = Date::<DefaultDateMapping<BasicDateTime>>::new(DateValue::build(2015, 05, 13, 0, 0, 0, 0));

        let ser = serde_json::to_string(&date).unwrap();

        assert_eq!(r#""20150513T000000.000Z""#, ser);
    }

    #[test]
    fn deserialise_elastic_date() {
        let date: Date<DefaultDateMapping<BasicDateTime>> = serde_json::from_str(r#""20150513T000000.000Z""#).unwrap();

        assert_eq!((2015, 5, 13), (date.year(), date.month(), date.day()));
    }

    #[test]
    fn serialise_date_expr_now() {
        let expr = DateExpr::<DefaultDateFormat>::now();

        let ser = serde_json::to_string(&expr).unwrap();

        assert_eq!(r#""now""#, ser);
    }

    #[test]
    fn serialise_date_expr_chrono() {
        let date = chrono::Utc.datetime_from_str("13/05/2015 00:00:00", "%d/%m/%Y %H:%M:%S").unwrap();

        let expr = DateExpr::value(date);

        let ser = serde_json::to_string(&expr).unwrap();

        assert_eq!(r#""2015-05-13T00:00:00Z||""#, ser);
    }

    #[test]
    fn serialise_date_expr_date() {
        let expr = DateExpr::value(Date::<DefaultDateMapping<BasicDateTime>>::build(2015, 5, 13, 0, 0, 0, 0));

        let ser = serde_json::to_string(&expr).unwrap();

        assert_eq!(r#""20150513T000000.000Z||""#, ser);
    }

    #[test]
    fn serialise_date_expr_value_with_ops() {
        let expr = DateExpr::value(Date::<DefaultDateMapping<BasicDateTime>>::build(2015, 5, 13, 0, 0, 0, 0))
            .add_days(2)
            .round_week();

        let ser = serde_json::to_string(&expr).unwrap();

        assert_eq!(r#""20150513T000000.000Z||+2d/w""#, ser);
    }

    #[test]
    fn serialise_date_expr_add() {
        let expr = DateExpr::<DefaultDateFormat>::now()
            .add_years(1)
            .add_months(2)
            .add_weeks(3)
            .add_days(4)
            .add_hours(5)
            .add_minutes(6)
            .add_seconds(7);

        let ser = serde_json::to_string(&expr).unwrap();

        assert_eq!(r#""now+1y+2M+3w+4d+5h+6m+7s""#, ser);
    }

    #[test]
    fn serialise_date_expr_sub() {
        let expr = DateExpr::<DefaultDateFormat>::now()
            .sub_years(1)
            .sub_months(2)
            .sub_weeks(3)
            .sub_days(4)
            .sub_hours(5)
            .sub_minutes(6)
            .sub_seconds(7);

        let ser = serde_json::to_string(&expr).unwrap();

        assert_eq!(r#""now-1y-2M-3w-4d-5h-6m-7s""#, ser);
    }

    #[test]
    fn serialise_date_expr_round() {
        let expr = DateExpr::<DefaultDateFormat>::now()
            .round_year()
            .round_month()
            .round_week()
            .round_day()
            .round_hour()
            .round_minute()
            .round_second();

        let ser = serde_json::to_string(&expr).unwrap();

        assert_eq!(r#""now/y/M/w/d/h/m/s""#, ser);
    }
}